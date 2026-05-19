use std::ops::Range;

use crate::style::Style;

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
    pub style: Style,
    pub padded: bool,
}

/// Merge overlapping spans into non-overlapping `ResolvedSpan`s.
///
/// `priorities[i]` is the priority of `spans[i]`; lower numbers win conflicts
/// (earlier finders take precedence). The two slices must be the same length.
///
/// `padded_ranges` must be sorted by start. Any resolved fragment whose
/// `(start, end)` exactly matches an entry in `padded_ranges` gets
/// `padded = true`. Fragments that don't match exactly get `padded = false`
/// — this is how a fragmented badge loses its surrounding spaces.
///
/// Returns spans sorted by position with no gaps or overlaps.
pub(crate) fn merge_spans(
    input_len: usize,
    spans: &[Span],
    priorities: &[usize],
    padded_ranges: &[Range<usize>],
) -> Vec<ResolvedSpan> {
    debug_assert_eq!(spans.len(), priorities.len());

    if spans.is_empty() {
        return Vec::new();
    }

    // Phase 1: Fill a style-per-byte array, lower priority wins.
    // Slot uses `u16` (not `usize`) to keep the byte-map dense — benchmarked
    // a 5% regression with `(Style, usize)` slots on a busy line, since this
    // is the hottest per-byte loop in the pipeline. `u16` gives 65k finder
    // headroom; realistic configs have ~13. Not pooled across calls: also
    // benchmarked, no measurable win.
    let mut style_map: Vec<Option<(Style, u16)>> = vec![None; input_len];

    for (span, &priority) in spans.iter().zip(priorities.iter()) {
        debug_assert!(u16::try_from(priority).is_ok(), "finder count exceeds u16 slot range");
        #[allow(clippy::cast_possible_truncation)]
        let priority = priority as u16;
        for slot in &mut style_map[span.start..span.end] {
            match slot {
                None => *slot = Some((span.style, priority)),
                Some((_, existing_pri)) if priority < *existing_pri => {
                    *slot = Some((span.style, priority));
                }
                _ => {}
            }
        }
    }

    // Phase 2: Run-length encode into ResolvedSpans, setting `padded` for
    // fragments whose endpoints exactly match a padded range. `pad_idx` slides
    // forward as we emit; both lists are sorted by start, so the scan is linear.
    let mut result = Vec::new();
    let mut pad_idx = 0;
    let mut i = 0;
    while i < input_len {
        if let Some((style, _)) = style_map[i] {
            let start = i;
            while i < input_len && style_map[i].is_some_and(|(s, _)| s == style) {
                i += 1;
            }
            while pad_idx < padded_ranges.len() && padded_ranges[pad_idx].end <= start {
                pad_idx += 1;
            }
            let padded = pad_idx < padded_ranges.len()
                && padded_ranges[pad_idx].start == start
                && padded_ranges[pad_idx].end == i;
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
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn red() -> Style {
        Style::new().fg(Color::Red)
    }

    fn blue() -> Style {
        Style::new().fg(Color::Blue)
    }

    fn yellow() -> Style {
        Style::new().fg(Color::Yellow)
    }

    #[test]
    fn empty_spans() {
        let result = merge_spans(10, &[], &[], &[]);
        assert!(result.is_empty());
    }

    fn resolved(start: usize, end: usize, style: Style) -> ResolvedSpan {
        ResolvedSpan {
            start,
            end,
            style,
            padded: false,
        }
    }

    fn padded(start: usize, end: usize, style: Style) -> ResolvedSpan {
        ResolvedSpan {
            start,
            end,
            style,
            padded: true,
        }
    }

    #[test]
    fn single_span() {
        let spans = [Span::new(2, 5, red())];
        let result = merge_spans(10, &spans, &[0], &[]);
        assert_eq!(result, vec![resolved(2, 5, red())]);
    }

    #[test]
    fn non_overlapping_spans() {
        let spans = [Span::new(0, 3, red()), Span::new(5, 8, blue())];
        let result = merge_spans(10, &spans, &[0, 1], &[]);
        assert_eq!(result, vec![resolved(0, 3, red()), resolved(5, 8, blue())]);
    }

    #[test]
    fn overlapping_higher_priority_wins() {
        // Red (priority 0) overlaps with blue (priority 1) at bytes 3-5
        let spans = [Span::new(0, 6, red()), Span::new(3, 8, blue())];
        let result = merge_spans(10, &spans, &[0, 1], &[]);
        assert_eq!(result, vec![resolved(0, 6, red()), resolved(6, 8, blue())]);
    }

    #[test]
    fn lower_priority_fills_gaps() {
        // Number (priority 0) at "42", quote (priority 1) wraps entire region
        let spans = [Span::new(5, 7, red()), Span::new(0, 10, yellow())];
        let result = merge_spans(10, &spans, &[0, 1], &[]);
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
        let spans = [Span::new(0, 3, red()), Span::new(3, 6, blue())];
        let result = merge_spans(6, &spans, &[0, 0], &[]);
        assert_eq!(result, vec![resolved(0, 3, red()), resolved(3, 6, blue())]);
    }

    #[test]
    fn intact_padded_span_keeps_padded_flag() {
        // Single keyword-style match that survives merge as-is.
        let spans = [Span::new(2, 7, red())];
        let result = merge_spans(10, &spans, &[0], &[2..7]);
        assert_eq!(result, vec![padded(2, 7, red())]);
    }

    #[test]
    fn fragmented_padded_span_loses_padded_flag() {
        // Keyword span 2..7 (priority 1), overridden in 4..5 by a higher-priority
        // finder (priority 0). Both fragments must have padded=false.
        let spans = [Span::new(4, 5, blue()), Span::new(2, 7, red())];
        let result = merge_spans(10, &spans, &[0, 1], &[2..7]);
        assert_eq!(
            result,
            vec![resolved(2, 4, red()), resolved(4, 5, blue()), resolved(5, 7, red()),]
        );
    }

    #[test]
    fn multiple_padded_ranges_in_order() {
        // Two badges far apart — both should survive padded.
        let spans = [Span::new(0, 4, red()), Span::new(6, 10, blue())];
        let result = merge_spans(10, &spans, &[0, 1], &[0..4, 6..10]);
        assert_eq!(result, vec![padded(0, 4, red()), padded(6, 10, blue())]);
    }
}
