use std::ops::Range;

use crate::style::Style;

/// A styled region within the original input text.
///
/// `start` and `end` are byte offsets into the original unstyled input.
/// Invariant: `start < end`, offsets are valid UTF-8 boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub start: usize,
    pub end: usize,
    pub style: Style,
    pub priority: u16,
}

#[cfg(test)]
impl Span {
    pub fn new(start: usize, end: usize, style: Style, priority: u16) -> Self {
        Self {
            start,
            end,
            style,
            priority,
        }
    }
}

/// Collects spans from a single finder, coalescing adjacent same-style spans.
#[derive(Debug)]
pub struct Collector {
    spans: Vec<Span>,
    padded_ranges: Vec<Range<usize>>,
    priority: u16,
}

impl Collector {
    pub fn new(priority: u16) -> Self {
        Self {
            spans: Vec::new(),
            padded_ranges: Vec::new(),
            priority,
        }
    }

    /// Push a span. If it is contiguous with the last span and has the same
    /// style, extend the last span rather than pushing a new one.
    pub fn push(&mut self, start: usize, end: usize, style: Style) {
        if let Some(last) = self.spans.last_mut()
            && last.style == style
            && last.end == start
        {
            last.end = end;
            return;
        }
        self.spans.push(Span {
            start,
            end,
            style,
            priority: self.priority,
        });
    }

    /// Push a span with padding. Render will insert a space before and after
    /// the span text, inside the ANSI color (creating a "badge" effect for
    /// keywords with background colors).
    pub fn push_padded(&mut self, start: usize, end: usize, style: Style) {
        self.push(start, end, style);
        self.padded_ranges.push(start..end);
    }

    #[cfg(test)]
    pub(crate) fn into_spans(self) -> Vec<Span> {
        self.spans
    }

    pub(crate) fn into_parts(self) -> (Vec<Span>, Vec<Range<usize>>) {
        (self.spans, self.padded_ranges)
    }
}

/// Trait for highlighters in the span-based pipeline.
///
/// Implementations run on the original unstyled input and push spans
/// into the collector.
pub trait Finder: Sync + Send {
    fn find_spans(&self, input: &str, collector: &mut Collector);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn coalesces_adjacent_same_style() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new(0);
        collector.push(0, 1, style);
        collector.push(1, 2, style);
        collector.push(2, 3, style);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].start, 0);
        assert_eq!(spans[0].end, 3);
    }

    #[test]
    fn does_not_coalesce_different_styles() {
        let red = Style::new().fg(Color::Red);
        let blue = Style::new().fg(Color::Blue);
        let mut collector = Collector::new(0);
        collector.push(0, 1, red);
        collector.push(1, 2, blue);
        collector.push(2, 3, red);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn does_not_coalesce_non_adjacent() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new(0);
        collector.push(0, 1, style);
        collector.push(3, 4, style);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn sets_priority_from_collector() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new(5);
        collector.push(0, 3, style);

        let spans = collector.into_spans();
        assert_eq!(spans[0].priority, 5);
    }
}
