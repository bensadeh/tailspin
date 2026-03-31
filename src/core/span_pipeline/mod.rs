pub(crate) mod finders;
pub(crate) mod merge;
pub(crate) mod render;
pub(crate) mod span;

use std::borrow::Cow;

use merge::merge_spans;
use render::render;
use span::{Collector, Finder};

/// Span-based highlighter pipeline.
///
/// All finders run on the original unstyled input and produce spans.
/// A merge step resolves overlaps by priority, and a single render pass
/// produces the ANSI-colored output.
pub struct Pipeline {
    finders: Vec<Box<dyn Finder>>,
}

impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipeline")
            .field("finders", &self.finders.len())
            .finish()
    }
}

impl Pipeline {
    pub fn new(finders: Vec<Box<dyn Finder>>) -> Self {
        Self { finders }
    }

    /// Apply all finders sequentially, merge, render.
    pub fn apply_sequential<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut all_spans = Vec::new();
        let mut padded_ranges = Vec::new();

        for (priority, finder) in self.finders.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let mut collector = Collector::new(priority as u16);
            finder.find_spans(input, &mut collector);
            let (spans, padded) = collector.into_parts();
            all_spans.extend(spans);
            padded_ranges.extend(padded);
        }

        padded_ranges.sort_unstable_by_key(|r| r.start);

        let resolved = merge_spans(input.len(), &all_spans);
        render(input, &resolved, &padded_ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::style::{Color, Style};
    use finders::keyword::KeywordFinder;
    use finders::number::NumberFinder;
    use finders::quote::QuoteFinder;

    #[test]
    fn end_to_end_number_highlighter() {
        let highlighter = Pipeline::new(vec![Box::new(NumberFinder::new(Style::new().fg(Color::Cyan)))]);

        let result = highlighter.apply_sequential("hello 42 world");
        assert_eq!(result.to_string().convert_escape_codes(), "hello [cyan]42[reset] world");
    }

    #[test]
    fn no_match_returns_borrowed() {
        let highlighter = Pipeline::new(vec![Box::new(NumberFinder::new(Style::new().fg(Color::Cyan)))]);

        let result = highlighter.apply_sequential("no numbers here");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn number_plus_quote_priority() {
        // Number (priority 0) should win inside quoted region (priority 1)
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        let result = highlighter.apply_sequential(r#"count is "value 42 here" end"#);
        let readable = result.to_string().convert_escape_codes();

        // Number 42 should be cyan, quote region should be yellow, outside should be unstyled
        assert_eq!(
            readable,
            r#"count is [yellow]"value [reset][cyan]42[reset][yellow] here"[reset] end"#
        );
    }

    #[test]
    fn multiple_numbers_inside_quotes() {
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        let result = highlighter.apply_sequential(r#""port 8080 and 443""#);
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(
            readable,
            r#"[yellow]"port [reset][cyan]8080[reset][yellow] and [reset][cyan]443[reset][yellow]"[reset]"#
        );
    }

    #[test]
    fn no_quotes_only_numbers() {
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        let result = highlighter.apply_sequential("status 200 ok");
        assert_eq!(result.to_string().convert_escape_codes(), "status [cyan]200[reset] ok");
    }

    #[test]
    fn keyword_with_background_gets_padding() {
        let highlighter = Pipeline::new(vec![Box::new(
            KeywordFinder::new(&["ERROR"], Style::new().on(Color::Red)).unwrap(),
        )]);

        let result = highlighter.apply_sequential("level ERROR here");
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "level [bg_red] ERROR [reset] here"
        );
    }

    #[test]
    fn keyword_without_background_no_padding() {
        let highlighter = Pipeline::new(vec![Box::new(
            KeywordFinder::new(&["ERROR"], Style::new().fg(Color::Red)).unwrap(),
        )]);

        let result = highlighter.apply_sequential("level ERROR here");
        assert_eq!(
            result.to_string().convert_escape_codes(),
            "level [red]ERROR[reset] here"
        );
    }

    #[test]
    fn multiple_keyword_groups_padding_out_of_position_order() {
        // Finder 0 matches "ERROR" (later in string), finder 1 matches "WARN" (earlier).
        // padded_ranges are collected in finder order, not position order.
        // Both must still get badge padding.
        let highlighter = Pipeline::new(vec![
            Box::new(KeywordFinder::new(&["ERROR"], Style::new().on(Color::Red)).unwrap()),
            Box::new(KeywordFinder::new(&["WARN"], Style::new().on(Color::Yellow)).unwrap()),
        ]);

        let result = highlighter.apply_sequential("WARN then ERROR");
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(readable, "[bg_yellow] WARN [reset] then [bg_red] ERROR [reset]");
    }

    #[test]
    fn three_keyword_groups_padding_interleaved() {
        // Three finders whose matches appear in reverse finder order by position.
        let highlighter = Pipeline::new(vec![
            Box::new(KeywordFinder::new(&["TRACE"], Style::new().on(Color::Blue)).unwrap()),
            Box::new(KeywordFinder::new(&["WARN"], Style::new().on(Color::Yellow)).unwrap()),
            Box::new(KeywordFinder::new(&["DEBUG"], Style::new().on(Color::Cyan)).unwrap()),
        ]);

        let result = highlighter.apply_sequential("DEBUG WARN TRACE");
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(
            readable,
            "[bg_cyan] DEBUG [reset] [bg_yellow] WARN [reset] [bg_blue] TRACE [reset]"
        );
    }

    #[test]
    fn empty_input_returns_borrowed() {
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(KeywordFinder::new(&["ERROR"], Style::new().on(Color::Red)).unwrap()),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        let result = highlighter.apply_sequential("");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(&*result, "");
    }

    #[test]
    fn three_finders_overlapping_same_region() {
        // Number (priority 0), keyword (priority 1), quote (priority 2) all cover "200"
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(KeywordFinder::new(&["200"], Style::new().fg(Color::Green)).unwrap()),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        // "200" is inside quotes, matched by all three finders — number (priority 0) wins
        let result = highlighter.apply_sequential(r#""status 200 ok""#);
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(
            readable,
            r#"[yellow]"status [reset][cyan]200[reset][yellow] ok"[reset]"#
        );
    }

    #[test]
    fn multibyte_utf8_with_numbers() {
        let highlighter = Pipeline::new(vec![Box::new(NumberFinder::new(Style::new().fg(Color::Cyan)))]);

        let result = highlighter.apply_sequential("café 42 résumé");
        assert_eq!(result.to_string().convert_escape_codes(), "café [cyan]42[reset] résumé");
    }

    #[test]
    fn multibyte_utf8_with_quotes() {
        let highlighter = Pipeline::new(vec![
            Box::new(NumberFinder::new(Style::new().fg(Color::Cyan))),
            Box::new(QuoteFinder::new(b'"', Style::new().fg(Color::Yellow))),
        ]);

        let result = highlighter.apply_sequential(r#"日本語 "hello 42" 世界"#);
        let readable = result.to_string().convert_escape_codes();
        assert_eq!(
            readable,
            r#"日本語 [yellow]"hello [reset][cyan]42[reset][yellow]"[reset] 世界"#
        );
    }

    #[test]
    fn keyword_badge_is_entire_input() {
        let highlighter = Pipeline::new(vec![Box::new(
            KeywordFinder::new(&["ERROR"], Style::new().on(Color::Red)).unwrap(),
        )]);

        let result = highlighter.apply_sequential("ERROR");
        assert_eq!(result.to_string().convert_escape_codes(), "[bg_red] ERROR [reset]");
    }

    #[test]
    fn ansi_input_passes_through() {
        // Pre-styled input: finders won't match inside ANSI codes,
        // but the pipeline should not panic or corrupt output
        let highlighter = Pipeline::new(vec![Box::new(NumberFinder::new(Style::new().fg(Color::Cyan)))]);

        let input = "\x1b[31mhello\x1b[0m 42";
        let result = highlighter.apply_sequential(input);
        let readable = result.to_string().convert_escape_codes();
        // The 42 is still highlighted; ANSI codes are treated as opaque text
        assert!(readable.contains("[cyan]42[reset]"));
    }
}
