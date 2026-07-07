use std::borrow::Cow;

use super::merge::ResolvedSpan;
use super::palette::Palette;

const RESET: &str = "\x1b[0m";

/// Render the original input with resolved spans into an ANSI-colored string.
///
/// Each span carries its own `padded` flag; render is a straight walk with
/// no cross-reference to a sibling buffer. The atomicity rule (a padded
/// match only keeps its surrounding spaces if merge preserved it whole) is
/// enforced upstream in `merge_spans` — render just reads the flag.
///
/// Returns `Cow::Borrowed` if no spans exist (zero allocation).
pub(crate) fn render<'a>(input: &'a str, spans: &[ResolvedSpan], palette: &Palette) -> Cow<'a, str> {
    if spans.is_empty() {
        return Cow::Borrowed(input);
    }

    let mut output = String::with_capacity(input.len() + spans.len() * 16);
    let mut pos = 0;

    for span in spans {
        if pos < span.start {
            output.push_str(&input[pos..span.start]);
        }

        output.push_str(&palette[span.style]);
        if span.padded {
            output.push(' ');
        }
        output.push_str(&input[span.start..span.end]);
        if span.padded {
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
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};

    use super::super::palette::StyleId;

    fn red(palette: &mut Palette) -> StyleId {
        palette.intern(Style::new().fg(Color::Red))
    }

    fn green(palette: &mut Palette) -> StyleId {
        palette.intern(Style::new().fg(Color::Green))
    }

    fn bg_red(palette: &mut Palette) -> StyleId {
        palette.intern(Style::new().on(Color::Red))
    }

    fn bg_yellow(palette: &mut Palette) -> StyleId {
        palette.intern(Style::new().on(Color::Yellow))
    }

    fn span(start: usize, end: usize, style: StyleId) -> ResolvedSpan {
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
    fn empty_spans_returns_borrowed() {
        let input = "hello world";
        let result = render(input, &[], &Palette::new());
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "hello world");
    }

    #[test]
    fn single_span_in_middle() {
        let input = "hello world";
        let mut palette = Palette::new();
        let red = red(&mut palette);
        let result = render(input, &[span(6, 11, red)], &palette);
        assert_eq!(result.to_string().convert_escape_codes(), "hello [red]world[reset]");
    }

    #[test]
    fn preserves_text_between_spans() {
        let input = "abc def ghi";
        let mut palette = Palette::new();
        let red = red(&mut palette);
        let result = render(input, &[span(0, 3, red), span(8, 11, red)], &palette);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[red]abc[reset] def [red]ghi[reset]"
        );
    }

    #[test]
    fn adjacent_spans_no_gap() {
        let input = "abcdef";
        let mut palette = Palette::new();
        let red = red(&mut palette);
        let green = green(&mut palette);
        let result = render(input, &[span(0, 3, red), span(3, 6, green)], &palette);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[red]abc[reset][green]def[reset]"
        );
    }

    #[test]
    fn padded_span_gets_spaces() {
        let input = "x ERROR y";
        let mut palette = Palette::new();
        let bg_red = bg_red(&mut palette);
        let result = render(input, &[padded(2, 7, bg_red)], &palette);
        assert_eq!(result.to_string().convert_escape_codes(), "x [bg_red] ERROR [reset] y");
    }

    #[test]
    fn padded_span_at_start_of_input() {
        let input = "ERROR rest";
        let mut palette = Palette::new();
        let bg_red = bg_red(&mut palette);
        let result = render(input, &[padded(0, 5, bg_red)], &palette);
        assert_eq!(result.to_string().convert_escape_codes(), "[bg_red] ERROR [reset] rest");
    }

    #[test]
    fn padded_span_at_end_of_input() {
        let input = "prefix ERROR";
        let mut palette = Palette::new();
        let bg_red = bg_red(&mut palette);
        let result = render(input, &[padded(7, 12, bg_red)], &palette);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "prefix [bg_red] ERROR [reset]"
        );
    }

    #[test]
    fn non_padded_span_gets_no_spaces() {
        let input = "x ERROR y";
        let mut palette = Palette::new();
        let red = red(&mut palette);
        let result = render(input, &[span(2, 7, red)], &palette);
        assert_eq!(result.to_string().convert_escape_codes(), "x [red]ERROR[reset] y");
    }

    #[test]
    fn mixed_padded_and_plain() {
        let input = "WARN then ERROR end";
        let mut palette = Palette::new();
        let bg_yellow = bg_yellow(&mut palette);
        let bg_red = bg_red(&mut palette);
        let result = render(input, &[padded(0, 4, bg_yellow), padded(10, 15, bg_red)], &palette);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[bg_yellow] WARN [reset] then [bg_red] ERROR [reset] end"
        );
    }

    #[test]
    fn padded_followed_by_plain_fragment() {
        // The kind of output merge produces when a higher-priority finder
        // splits a badge: first fragment is plain-styled (no padding), second
        // is also plain.
        let input = "x ERROR y";
        let mut palette = Palette::new();
        let bg_red = bg_red(&mut palette);
        let green = green(&mut palette);
        let result = render(input, &[span(2, 5, bg_red), span(5, 7, green)], &palette);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "x [bg_red]ERR[reset][green]OR[reset] y"
        );
    }
}
