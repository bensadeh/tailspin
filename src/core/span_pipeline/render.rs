use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;

use nu_ansi_term::Style as NuStyle;

use crate::style::Style;

use super::merge::ResolvedSpan;

const RESET: &str = "\x1b[0m";

thread_local! {
    static PREFIX_CACHE: RefCell<HashMap<Style, String>> = RefCell::new(HashMap::new());
}

fn compute_prefix(style: Style) -> String {
    let nu: NuStyle = style.into();
    nu.prefix().to_string()
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

    PREFIX_CACHE.with_borrow_mut(|cache| {
        let mut output = String::with_capacity(input.len() + spans.len() * 16);
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
            // Only pad if this span covers the entire padded range;
            // fragments produced by higher-priority finders get no padding.
            let padded = pad_idx < padded_ranges.len()
                && padded_ranges[pad_idx].start == span.start
                && span.end == padded_ranges[pad_idx].end;

            let prefix = cache.entry(span.style).or_insert_with(|| compute_prefix(span.style));
            output.push_str(prefix);
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
    })
}

#[cfg(test)]
#[allow(clippy::single_range_in_vec_init)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
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
        assert_eq!(result.to_string().convert_escape_codes(), "hello [red]world[reset]");
    }

    #[test]
    fn preserves_text_between_spans() {
        let input = "abc def ghi";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(0, 3, style), span(8, 11, style)], &[]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[red]abc[reset] def [red]ghi[reset]"
        );
    }

    #[test]
    fn adjacent_spans_no_gap() {
        let input = "abcdef";
        let red = Style::new().fg(Color::Red);
        let blue = Style::new().fg(Color::Blue);
        let result = render(input, &[span(0, 3, red), span(3, 6, blue)], &[]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[red]abc[reset][blue]def[reset]"
        );
    }

    #[test]
    fn padded_span_gets_spaces() {
        let input = "x ERROR y";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[span(2, 7, style)], &[Range { start: 2, end: 7 }]);
        assert_eq!(result.to_string().convert_escape_codes(), "x [bg_red] ERROR [reset] y");
    }

    #[test]
    fn fragmented_padded_span_gets_no_spaces() {
        // When a higher-priority finder splits a padded keyword, the fragments
        // should NOT receive padding — only complete badges get spaces.
        let input = "x ERROR y";
        let keyword = Style::new().on(Color::Red);
        let number = Style::new().fg(Color::Green);
        // Simulate merge splitting "ERROR" (2..7) because bytes 5..7 were claimed
        // by a higher-priority finder.
        let spans = &[span(2, 5, keyword), span(5, 7, number)];
        let padded = &[2..7];
        let result = render(input, spans, padded);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "x [bg_red]ERR[reset][green]OR[reset] y"
        );
    }

    #[test]
    fn padded_span_at_start_of_input() {
        let input = "ERROR rest";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[span(0, 5, style)], &[0..5]);
        assert_eq!(result.to_string().convert_escape_codes(), "[bg_red] ERROR [reset] rest");
    }

    #[test]
    fn padded_span_at_end_of_input() {
        let input = "prefix ERROR";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[span(7, 12, style)], &[7..12]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "prefix [bg_red] ERROR [reset]"
        );
    }

    #[test]
    fn fragmented_padded_span_from_left() {
        // Higher-priority finder overrides the start of a padded keyword.
        let input = "x ERROR y";
        let keyword = Style::new().on(Color::Red);
        let other = Style::new().fg(Color::Green);
        let spans = &[span(2, 4, other), span(4, 7, keyword)];
        let padded = &[2..7];
        let result = render(input, spans, padded);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "x [green]ER[reset][bg_red]ROR[reset] y"
        );
    }

    #[test]
    fn fragmented_padded_span_in_middle() {
        // Higher-priority finder overrides the middle of a padded keyword,
        // producing two keyword fragments. Neither should get padding.
        let input = "x ERROR y";
        let keyword = Style::new().on(Color::Red);
        let other = Style::new().fg(Color::Green);
        let spans = &[span(2, 4, keyword), span(4, 5, other), span(5, 7, keyword)];
        let padded = &[2..7];
        let result = render(input, spans, padded);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "x [bg_red]ER[reset][green]R[reset][bg_red]OR[reset] y"
        );
    }

    #[test]
    fn non_padded_span_gets_no_spaces() {
        let input = "x ERROR y";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(2, 7, style)], &[]);
        assert_eq!(result.to_string().convert_escape_codes(), "x [red]ERROR[reset] y");
    }

    #[test]
    fn multiple_padded_ranges() {
        let input = "WARN then ERROR end";
        let yellow = Style::new().on(Color::Yellow);
        let red = Style::new().on(Color::Red);
        let spans = &[span(0, 4, yellow), span(10, 15, red)];
        let padded = &[0..4, 10..15];
        let result = render(input, spans, padded);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[bg_yellow] WARN [reset] then [bg_red] ERROR [reset] end"
        );
    }
}
