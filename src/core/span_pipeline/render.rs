use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;

use nu_ansi_term::Style as NuStyle;

use crate::style::Style;

use super::merge::ResolvedSpan;

const RESET: &str = "\x1b[0m";

/// Cache of `Style` -> ANSI prefix string, computed lazily.
struct PrefixCache {
    cache: HashMap<Style, String>,
}

impl PrefixCache {
    fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    fn get(&mut self, style: Style) -> &str {
        self.cache.entry(style).or_insert_with(|| {
            let nu: NuStyle = style.into();
            let styled = format!("{}", nu.paint(""));
            styled.replace(RESET, "")
        })
    }
}

/// Render the original input with resolved spans into an ANSI-colored string.
///
/// `padded_ranges` are byte ranges (from the original input) that should be
/// rendered with a space before and after the text, inside the ANSI color.
///
/// Returns `Cow::Borrowed` if no spans exist (zero allocation).
pub(crate) fn render<'a>(input: &'a str, spans: &[ResolvedSpan], padded_ranges: &[Range<usize>]) -> Cow<'a, str> {
    if spans.is_empty() {
        return Cow::Borrowed(input);
    }

    let mut output = String::with_capacity(input.len() + spans.len() * 16);
    let mut cache = PrefixCache::new();
    let mut pos = 0;
    let mut pad_idx = 0;

    for span in spans {
        if pos < span.start {
            output.push_str(&input[pos..span.start]);
        }

        // Advance past padded ranges that end before this span
        while pad_idx < padded_ranges.len() && padded_ranges[pad_idx].end <= span.start {
            pad_idx += 1;
        }
        let padded = pad_idx < padded_ranges.len()
            && padded_ranges[pad_idx].start <= span.start
            && span.end <= padded_ranges[pad_idx].end;

        let prefix = cache.get(span.style).to_owned();
        output.push_str(&prefix);
        if padded {
            output.push(' ');
        }
        output.push_str(&input[span.start..span.end]);
        if padded {
            output.push(' ');
        }
        output.push_str(RESET);

        pos = span.end;
    }

    if pos < input.len() {
        output.push_str(&input[pos..]);
    }

    Cow::Owned(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn span(start: usize, end: usize, style: Style) -> ResolvedSpan {
        ResolvedSpan { start, end, style }
    }

    #[test]
    fn empty_spans_returns_borrowed() {
        let input = "hello world";
        let result = render(input, &[], &[]);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "hello world");
    }

    #[test]
    fn single_span_in_middle() {
        let input = "hello world";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(6, 11, style)], &[]);
        assert!(result.starts_with("hello "));
        assert!(result.contains("world"));
        assert!(result.contains(RESET));
    }

    #[test]
    fn preserves_text_between_spans() {
        let input = "abc def ghi";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(0, 3, style), span(8, 11, style)], &[]);
        assert!(result.contains(" def "));
    }

    #[test]
    fn adjacent_spans_no_gap() {
        let input = "abcdef";
        let red = Style::new().fg(Color::Red);
        let blue = Style::new().fg(Color::Blue);
        let result = render(input, &[span(0, 3, red), span(3, 6, blue)], &[]);
        // Should contain two separate styled regions
        let reset_count = result.matches(RESET).count();
        assert_eq!(reset_count, 2);
    }

    #[test]
    fn padded_span_gets_spaces() {
        let input = "x ERROR y";
        let style = Style::new().on(Color::Red);
        let spans = &[span(2, 7, style)];
        let padded = &[2..7];
        let result = render(input, spans, padded);
        // Should have space before and after "ERROR" inside the ANSI color
        assert!(result.contains(" ERROR "));
        assert!(result.contains(RESET));
    }

    #[test]
    fn non_padded_span_gets_no_spaces() {
        let input = "x ERROR y";
        let style = Style::new().fg(Color::Red);
        let spans = &[span(2, 7, style)];
        let result = render(input, spans, &[]);
        // No extra spaces — just the styled text
        assert!(!result.contains(" ERROR "));
    }

    #[test]
    fn multiple_padded_ranges_sorted() {
        // Two padded ranges in position order — both should get padding
        let input = "WARN then ERROR end";
        let yellow = Style::new().on(Color::Yellow);
        let red = Style::new().on(Color::Red);
        let spans = &[span(0, 4, yellow), span(10, 15, red)];
        let padded = &[0..4, 10..15];
        let result = render(input, spans, padded);
        assert!(result.contains(" WARN "));
        assert!(result.contains(" ERROR "));
    }
}
