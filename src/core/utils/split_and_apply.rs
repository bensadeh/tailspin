use crate::core::highlighter::Highlight;
use memchr::memmem::Finder;
use std::borrow::Cow;
use std::cmp::min;
use std::sync::LazyLock;

static ESCAPE_FINDER: LazyLock<Finder<'static>> = LazyLock::new(|| Finder::new("\x1b["));
static RESET_FINDER: LazyLock<Finder<'static>> = LazyLock::new(|| Finder::new("\x1b[0m"));

const RESET_LEN: usize = "\x1b[0m".len();
const SIXTEEN_KB: usize = 16 * 1024;

pub fn apply_only_to_unhighlighted<'a>(input: &'a str, highlighter: &(impl Highlight + ?Sized)) -> Cow<'a, str> {
    let bytes = input.as_bytes();
    let mut result: Option<String> = None;
    let mut copied = 0usize;
    let mut pos = 0;

    while pos < bytes.len() {
        match ESCAPE_FINDER.find(&bytes[pos..]) {
            None => {
                apply_chunk(&input[pos..], highlighter, input, &mut result, &mut copied);
                break;
            }
            Some(esc_offset) => {
                if esc_offset > 0 {
                    apply_chunk(
                        &input[pos..pos + esc_offset],
                        highlighter,
                        input,
                        &mut result,
                        &mut copied,
                    );
                    pos += esc_offset;
                }

                if let Some(reset_offset) = RESET_FINDER.find(&bytes[pos..]) {
                    let end = pos + reset_offset + RESET_LEN;
                    push_unchanged(&input[pos..end], &mut result, &mut copied);
                    pos = end;
                } else {
                    push_unchanged(&input[pos..], &mut result, &mut copied);
                    break;
                }
            }
        }
    }

    result.map_or(Cow::Borrowed(input), Cow::Owned)
}

#[inline(always)]
fn apply_chunk(
    text: &str,
    highlighter: &(impl Highlight + ?Sized),
    input: &str,
    result: &mut Option<String>,
    copied: &mut usize,
) {
    match highlighter.apply(text) {
        Cow::Borrowed(_) => push_unchanged(text, result, copied),
        Cow::Owned(ref new_text) => push_changed(new_text, input, result, copied),
    }
}

#[inline(always)]
fn push_unchanged(text: &str, result: &mut Option<String>, copied: &mut usize) {
    if let Some(buf) = result {
        buf.push_str(text);
    } else {
        *copied += text.len();
    }
}

#[inline(always)]
fn push_changed(new_text: &str, input: &str, result: &mut Option<String>, copied: &mut usize) {
    let buf = result.get_or_insert_with(|| {
        let extra = min(input.len(), SIXTEEN_KB);
        let mut s = String::with_capacity(input.len() + extra);
        s.push_str(&input[..*copied]);
        s
    });
    buf.push_str(new_text);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::NumberConfig;
    use crate::core::highlighters::number::NumberHighlighter;
    use crate::style::{Color, Style};

    fn number_highlighter() -> NumberHighlighter {
        NumberHighlighter::new(NumberConfig {
            style: Style::new().fg(Color::Cyan),
        })
    }

    #[test]
    fn no_escapes_no_match() {
        let h = number_highlighter();
        let input = "hello world";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "hello world");
    }

    #[test]
    fn no_escapes_with_match() {
        let h = number_highlighter();
        let input = "count 42 end";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Owned(_)));
        assert!(result.contains("\x1b["));
        assert!(result.contains("42"));
    }

    #[test]
    fn skips_already_highlighted_region() {
        let h = number_highlighter();
        let input = "before \x1b[31m99\x1b[0m after";
        let result = apply_only_to_unhighlighted(input, &h);
        // 99 is inside an escape — should NOT be re-highlighted
        // "before" and "after" have no numbers — no change
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn highlights_outside_escapes() {
        let h = number_highlighter();
        let input = "val 7 \x1b[31mred\x1b[0m end";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Owned(_)));
        // The "7" outside the escape should be highlighted
        assert!(result.contains("7"));
        // The escape region should be preserved verbatim
        assert!(result.contains("\x1b[31mred\x1b[0m"));
    }

    #[test]
    fn escape_at_start() {
        let h = number_highlighter();
        let input = "\x1b[31mred\x1b[0m 42 end";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Owned(_)));
        assert!(result.starts_with("\x1b[31mred\x1b[0m"));
    }

    #[test]
    fn escape_at_end() {
        let h = number_highlighter();
        let input = "hello \x1b[31m99\x1b[0m";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn unclosed_escape() {
        let h = number_highlighter();
        let input = "ok \x1b[31mforever red 42";
        let result = apply_only_to_unhighlighted(input, &h);
        // "ok " has no number, and everything after \x1b[ is treated as highlighted
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn multiple_escapes() {
        let h = number_highlighter();
        let input = "a \x1b[31mx\x1b[0m 5 \x1b[32my\x1b[0m b";
        let result = apply_only_to_unhighlighted(input, &h);
        assert!(matches!(result, Cow::Owned(_)));
        assert!(result.contains("\x1b[31mx\x1b[0m"));
        assert!(result.contains("\x1b[32my\x1b[0m"));
    }

    #[test]
    fn empty_input() {
        let h = number_highlighter();
        let result = apply_only_to_unhighlighted("", &h);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "");
    }
}
