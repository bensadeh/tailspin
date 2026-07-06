use crate::style::Style;

/// A styled region within the original input text.
///
/// `start` and `end` are byte offsets into the original unstyled input.
/// Invariant: `start < end`, offsets are valid UTF-8 boundaries.
///
/// `priority` is the index of the finder that produced the span; lower numbers
/// win conflicts in merge. It is `0` on a span freshly built by a `Collector`
/// and stamped with the real value when the pipeline drains the collector (see
/// [`Collector::drain_into`]).
///
/// `padded` asks render to surround the span text with a space on each side (a
/// "badge"), but only if merge preserves the span intact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub start: usize,
    pub end: usize,
    pub style: Style,
    pub priority: u16,
    pub padded: bool,
}

#[cfg(test)]
impl Span {
    pub fn new(start: usize, end: usize, style: Style, priority: u16) -> Self {
        Self {
            start,
            end,
            style,
            priority,
            padded: false,
        }
    }
}

/// Collects spans from a single finder, coalescing adjacent same-style spans.
#[derive(Debug)]
pub(crate) struct Collector {
    spans: Vec<Span>,
}

impl Collector {
    pub const fn new() -> Self {
        Self { spans: Vec::new() }
    }

    /// Push a span. If it is contiguous with the last span and shares its style
    /// and padding, extend the last span rather than pushing a new one.
    pub fn push(&mut self, start: usize, end: usize, style: Style) {
        self.push_impl(start, end, style, false);
    }

    /// Push a span with padding. Render will insert a space before and after
    /// the span text, inside the ANSI color (creating a "badge" effect for
    /// keywords with background colors).
    pub fn push_padded(&mut self, start: usize, end: usize, style: Style) {
        self.push_impl(start, end, style, true);
    }

    fn push_impl(&mut self, start: usize, end: usize, style: Style, padded: bool) {
        if start >= end {
            return;
        }

        // Coalesce only into a span that shares both style and padding. A padded
        // badge must not merge with an adjacent same-style plain span, because
        // merge decides padding by whole-span extent — a merged span would
        // either over- or under-pad.
        if let Some(last) = self.spans.last_mut()
            && last.style == style
            && last.padded == padded
            && last.end == start
        {
            last.end = end;
            return;
        }
        self.spans.push(Span {
            start,
            end,
            style,
            priority: 0,
            padded,
        });
    }

    #[cfg(test)]
    pub(crate) fn into_spans(self) -> Vec<Span> {
        self.spans
    }

    pub fn reset(&mut self) {
        self.spans.clear();
    }

    /// Append this collector's spans to `spans`, stamping each with `priority`
    /// (the producing finder's index). Leaves the collector empty for reuse.
    pub fn drain_into(&mut self, spans: &mut Vec<Span>, priority: u16) {
        for span in &mut self.spans {
            span.priority = priority;
        }
        spans.append(&mut self.spans);
    }
}

/// Trait for highlighters in the span-based pipeline.
///
/// Implementations run on the original unstyled input and push spans
/// into the collector.
pub(crate) trait Finder: std::fmt::Debug + Sync + Send + BoxedCloneFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector);
}

/// Object-safe clone for boxed finders, blanket-implemented so finder
/// types only need `#[derive(Clone)]`.
pub(crate) trait BoxedCloneFinder {
    fn boxed_clone(&self) -> Box<dyn Finder>;
}

impl<T: Finder + Clone + 'static> BoxedCloneFinder for T {
    fn boxed_clone(&self) -> Box<dyn Finder> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Finder> {
    fn clone(&self) -> Self {
        self.as_ref().boxed_clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn coalesces_adjacent_same_style() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new();
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
        let mut collector = Collector::new();
        collector.push(0, 1, red);
        collector.push(1, 2, blue);
        collector.push(2, 3, red);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn does_not_coalesce_non_adjacent() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new();
        collector.push(0, 1, style);
        collector.push(3, 4, style);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
    }

    #[test]
    fn reset_clears_spans() {
        let style = Style::new().fg(Color::Red);

        let mut collector = Collector::new();
        collector.push_padded(0, 3, style);
        collector.reset();
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn push_padded_marks_span_padded() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new();
        collector.push_padded(0, 3, style);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 1);
        assert!(spans[0].padded);
    }

    #[test]
    fn does_not_coalesce_padded_with_plain() {
        let style = Style::new().fg(Color::Red);
        let mut collector = Collector::new();
        collector.push(0, 1, style);
        collector.push_padded(1, 2, style);

        let spans = collector.into_spans();
        assert_eq!(spans.len(), 2);
        assert!(!spans[0].padded);
        assert!(spans[1].padded);
    }
}
