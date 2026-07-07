use super::palette::StyleId;
use super::span::Span;

/// A resolved style assignment for a contiguous range.
///
/// `padded` is `true` when this fragment is the entirety of an original
/// padded match (a "badge" that survived merge intact). Fragments produced
/// by a higher-priority finder splitting a padded match are emitted with
/// `padded = false` — they keep their style but lose the surrounding spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedSpan {
    pub start: usize,
    pub end: usize,
    pub style: StyleId,
    pub padded: bool,
}

/// Merge overlapping spans into non-overlapping `ResolvedSpan`s.
///
/// Each span carries its own `priority` (lower numbers win conflicts, so
/// earlier finders take precedence) and `padded` flag. Any resolved fragment
/// whose `(start, end)` exactly matches a padded span gets `padded = true`;
/// fragments that don't match exactly get `padded = false` — this is how a
/// fragmented badge loses its surrounding spaces.
///
/// Returns spans sorted by position with no gaps or overlaps.
pub(crate) fn merge_spans(input_len: usize, spans: &[Span]) -> Vec<ResolvedSpan> {
    if spans.is_empty() {
        return Vec::new();
    }

    // Phase 1: Fill a style-per-byte array, lower priority wins. The slot is
    // two u16s to keep the byte-map dense — this is the hottest per-byte loop
    // in the pipeline. Not pooled across calls: benchmarked, no measurable win.
    let mut style_map: Vec<Option<(StyleId, u16)>> = vec![None; input_len];

    for span in spans {
        for slot in &mut style_map[span.start..span.end] {
            match slot {
                None => *slot = Some((span.style, span.priority)),
                Some((_, existing_pri)) if span.priority < *existing_pri => {
                    *slot = Some((span.style, span.priority));
                }
                _ => {}
            }
        }
    }

    // Padded spans, sorted for the exact-match lookup in phase 2. Padding is
    // rare (only background-styled keywords), so this is usually empty and
    // allocates nothing.
    let mut padded_ranges: Vec<(usize, usize)> = spans.iter().filter(|s| s.padded).map(|s| (s.start, s.end)).collect();
    padded_ranges.sort_unstable();

    // Phase 2: Run-length encode into ResolvedSpans, setting `padded` for
    // fragments whose endpoints exactly match a padded span.
    let mut result = Vec::new();
    let mut i = 0;
    while i < input_len {
        if let Some((style, _)) = style_map[i] {
            let start = i;
            while i < input_len && style_map[i].is_some_and(|(s, _)| s == style) {
                i += 1;
            }
            let padded = padded_ranges.binary_search(&(start, i)).is_ok();
            result.push(ResolvedSpan {
                start,
                end: i,
                style,
                padded,
            });
        } else {
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn red() -> StyleId {
        StyleId::new(0)
    }

    fn blue() -> StyleId {
        StyleId::new(1)
    }

    fn yellow() -> StyleId {
        StyleId::new(2)
    }

    fn padded_span(start: usize, end: usize, style: StyleId, priority: u16) -> Span {
        Span {
            start,
            end,
            style,
            priority,
            padded: true,
        }
    }

    #[test]
    fn empty_spans() {
        let result = merge_spans(10, &[]);
        assert!(result.is_empty());
    }

    fn resolved(start: usize, end: usize, style: StyleId) -> ResolvedSpan {
        ResolvedSpan {
            start,
            end,
            style,
            padded: false,
        }
    }

    fn padded(start: usize, end: usize, style: StyleId) -> ResolvedSpan {
        ResolvedSpan {
            start,
            end,
            style,
            padded: true,
        }
    }

    #[test]
    fn single_span() {
        let spans = [Span::new(2, 5, red(), 0)];
        let result = merge_spans(10, &spans);
        assert_eq!(result, vec![resolved(2, 5, red())]);
    }

    #[test]
    fn non_overlapping_spans() {
        let spans = [Span::new(0, 3, red(), 0), Span::new(5, 8, blue(), 1)];
        let result = merge_spans(10, &spans);
        assert_eq!(result, vec![resolved(0, 3, red()), resolved(5, 8, blue())]);
    }

    #[test]
    fn overlapping_higher_priority_wins() {
        // Red (priority 0) overlaps with blue (priority 1) at bytes 3-5
        let spans = [Span::new(0, 6, red(), 0), Span::new(3, 8, blue(), 1)];
        let result = merge_spans(10, &spans);
        assert_eq!(result, vec![resolved(0, 6, red()), resolved(6, 8, blue())]);
    }

    #[test]
    fn lower_priority_fills_gaps() {
        // Number (priority 0) at "42", quote (priority 1) wraps entire region
        let spans = [Span::new(5, 7, red(), 0), Span::new(0, 10, yellow(), 1)];
        let result = merge_spans(10, &spans);
        assert_eq!(
            result,
            vec![
                resolved(0, 5, yellow()),
                resolved(5, 7, red()),
                resolved(7, 10, yellow())
            ]
        );
    }

    #[test]
    fn adjacent_different_styles() {
        let spans = [Span::new(0, 3, red(), 0), Span::new(3, 6, blue(), 0)];
        let result = merge_spans(6, &spans);
        assert_eq!(result, vec![resolved(0, 3, red()), resolved(3, 6, blue())]);
    }

    #[test]
    fn intact_padded_span_keeps_padded_flag() {
        // Single keyword-style match that survives merge as-is.
        let spans = [padded_span(2, 7, red(), 0)];
        let result = merge_spans(10, &spans);
        assert_eq!(result, vec![padded(2, 7, red())]);
    }

    #[test]
    fn fragmented_padded_span_loses_padded_flag() {
        // Padded span 2..7 (priority 1), overridden in 4..5 by a higher-priority
        // finder (priority 0). Both fragments must have padded=false.
        let spans = [Span::new(4, 5, blue(), 0), padded_span(2, 7, red(), 1)];
        let result = merge_spans(10, &spans);
        assert_eq!(
            result,
            vec![resolved(2, 4, red()), resolved(4, 5, blue()), resolved(5, 7, red()),]
        );
    }

    #[test]
    fn multiple_padded_ranges_in_order() {
        // Two badges far apart — both should survive padded.
        let spans = [padded_span(0, 4, red(), 0), padded_span(6, 10, blue(), 1)];
        let result = merge_spans(10, &spans);
        assert_eq!(result, vec![padded(0, 4, red()), padded(6, 10, blue())]);
    }

    #[test]
    fn overlapping_padded_ranges_keep_padding_on_the_intact_one() {
        // Padded span 4..13 (priority 0) wins its whole range and stays
        // intact; padded span 0..9 (priority 1) is fragmented down to 0..4.
        let spans = [padded_span(4, 13, red(), 0), padded_span(0, 9, blue(), 1)];
        let result = merge_spans(13, &spans);
        assert_eq!(result, vec![resolved(0, 4, blue()), padded(4, 13, red())]);
    }
}
