use crate::style::Style;

use super::span::Span;

/// A resolved style assignment for a contiguous range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedSpan {
    pub start: usize,
    pub end: usize,
    pub style: Style,
}

/// Merge overlapping spans into non-overlapping `ResolvedSpan`s.
///
/// Lower `priority` number wins conflicts (earlier highlighters take precedence).
/// Returns spans sorted by position with no gaps or overlaps.
pub(crate) fn merge_spans(input_len: usize, spans: &[Span]) -> Vec<ResolvedSpan> {
    if spans.is_empty() {
        return Vec::new();
    }

    // Phase 1: Fill a style-per-byte array, lower priority wins
    let mut style_map: Vec<Option<(Style, u16)>> = vec![None; input_len];

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

    // Phase 2: Run-length encode into ResolvedSpans
    let mut result = Vec::new();
    let mut i = 0;
    while i < input_len {
        if let Some((style, _)) = style_map[i] {
            let start = i;
            while i < input_len && style_map[i].is_some_and(|(s, _)| s == style) {
                i += 1;
            }
            result.push(ResolvedSpan { start, end: i, style });
        } else {
            i += 1;
        }
    }

    result
}

#[cfg(test)]
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
        let result = merge_spans(10, &[]);
        assert!(result.is_empty());
    }

    fn resolved(start: usize, end: usize, style: Style) -> ResolvedSpan {
        ResolvedSpan { start, end, style }
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
}
