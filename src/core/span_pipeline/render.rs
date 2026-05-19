use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

use nu_ansi_term::Style as NuStyle;

use crate::style::Style;

use super::merge::ResolvedSpan;

const RESET: &str = "\x1b[0m";

// BTreeMap (not HashMap) because Style is small and Ord; we hit this on every
// span emit, and SipHash showed up at ~9% of work in profiles. Tree lookup over
// the few-dozen distinct styles is faster than rehashing.
thread_local! {
    static PREFIX_CACHE: RefCell<BTreeMap<Style, String>> = const { RefCell::new(BTreeMap::new()) };
}

fn compute_prefix(style: Style) -> String {
    let nu: NuStyle = style.into();
    nu.prefix().to_string()
}

/// Render the original input with resolved spans into an ANSI-colored string.
///
/// Each span carries its own `padded` flag; render is a straight walk with
/// no cross-reference to a sibling buffer. The atomicity rule (a padded
/// match only keeps its surrounding spaces if merge preserved it whole) is
/// enforced upstream in `merge_spans` — render just reads the flag.
///
/// Returns `Cow::Borrowed` if no spans exist (zero allocation).
pub(crate) fn render<'a>(input: &'a str, spans: &[ResolvedSpan]) -> Cow<'a, str> {
    if spans.is_empty() {
        return Cow::Borrowed(input);
    }

    PREFIX_CACHE.with_borrow_mut(|cache| {
        let mut output = String::with_capacity(input.len() + spans.len() * 16);
        let mut pos = 0;

        for span in spans {
            if pos < span.start {
                output.push_str(&input[pos..span.start]);
            }

            let prefix = cache.entry(span.style).or_insert_with(|| compute_prefix(span.style));
            output.push_str(prefix);
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::Color;

    fn span(start: usize, end: usize, style: Style) -> ResolvedSpan {
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
    fn empty_spans_returns_borrowed() {
        let input = "hello world";
        let result = render(input, &[]);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "hello world");
    }

    #[test]
    fn single_span_in_middle() {
        let input = "hello world";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(6, 11, style)]);
        assert_eq!(result.to_string().convert_escape_codes(), "hello [red]world[reset]");
    }

    #[test]
    fn preserves_text_between_spans() {
        let input = "abc def ghi";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(0, 3, style), span(8, 11, style)]);
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
        let result = render(input, &[span(0, 3, red), span(3, 6, blue)]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "[red]abc[reset][blue]def[reset]"
        );
    }

    #[test]
    fn padded_span_gets_spaces() {
        let input = "x ERROR y";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[padded(2, 7, style)]);
        assert_eq!(result.to_string().convert_escape_codes(), "x [bg_red] ERROR [reset] y");
    }

    #[test]
    fn padded_span_at_start_of_input() {
        let input = "ERROR rest";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[padded(0, 5, style)]);
        assert_eq!(result.to_string().convert_escape_codes(), "[bg_red] ERROR [reset] rest");
    }

    #[test]
    fn padded_span_at_end_of_input() {
        let input = "prefix ERROR";
        let style = Style::new().on(Color::Red);
        let result = render(input, &[padded(7, 12, style)]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "prefix [bg_red] ERROR [reset]"
        );
    }

    #[test]
    fn non_padded_span_gets_no_spaces() {
        let input = "x ERROR y";
        let style = Style::new().fg(Color::Red);
        let result = render(input, &[span(2, 7, style)]);
        assert_eq!(result.to_string().convert_escape_codes(), "x [red]ERROR[reset] y");
    }

    #[test]
    fn mixed_padded_and_plain() {
        let input = "WARN then ERROR end";
        let yellow = Style::new().on(Color::Yellow);
        let red = Style::new().on(Color::Red);
        let result = render(input, &[padded(0, 4, yellow), padded(10, 15, red)]);
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
        let keyword = Style::new().on(Color::Red);
        let other = Style::new().fg(Color::Green);
        let result = render(input, &[span(2, 5, keyword), span(5, 7, other)]);
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "x [bg_red]ERR[reset][green]OR[reset] y"
        );
    }
}
