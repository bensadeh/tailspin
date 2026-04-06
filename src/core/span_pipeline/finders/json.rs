use serde::Deserialize;

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub(crate) struct JsonFinder {
    key: Style,
    quote_token: Style,
    curly_bracket: Style,
    square_bracket: Style,
    comma: Style,
    colon: Style,
}

impl JsonFinder {
    pub fn new(
        key: Style,
        quote_token: Style,
        curly_bracket: Style,
        square_bracket: Style,
        comma: Style,
        colon: Style,
    ) -> Self {
        Self {
            key,
            quote_token,
            curly_bracket,
            square_bracket,
            comma,
            colon,
        }
    }
}

impl Finder for JsonFinder {
    fn find_spans(&self, input: &str, collector: &mut Collector) {
        let first = input.as_bytes().iter().find(|b| !b.is_ascii_whitespace());
        if first != Some(&b'{') && first != Some(&b'[') {
            return;
        }

        // JSON highlighting rewrites the entire input (reformatting whitespace),
        // so we can't produce byte-offset spans into the original input.
        // Instead, we style the structural tokens at their original positions
        // by walking the raw input string.
        //
        // Validate it's JSON without allocating the tree.
        let mut de = serde_json::Deserializer::from_str(input);
        if serde::de::IgnoredAny::deserialize(&mut de).is_err() || de.end().is_err() {
            return;
        }

        let bytes = input.as_bytes();
        let mut in_string = false;
        let mut escape_next = false;

        let mut i = 0;
        while i < bytes.len() {
            let b = bytes[i];

            if escape_next {
                escape_next = false;
                i += 1;
                continue;
            }

            if in_string {
                if b == b'\\' {
                    escape_next = true;
                } else if b == b'"' {
                    collector.push(i, i + 1, self.quote_token);
                    in_string = false;
                }
                i += 1;
                continue;
            }

            match b {
                b'{' | b'}' => collector.push(i, i + 1, self.curly_bracket),
                b'[' | b']' => collector.push(i, i + 1, self.square_bracket),
                b',' => collector.push(i, i + 1, self.comma),
                b':' => collector.push(i, i + 1, self.colon),
                b'"' => {
                    collector.push(i, i + 1, self.quote_token);
                    in_string = true;

                    // A quoted string is a key unless preceded by `:`.
                    // Only whitespace can appear between structural tokens in
                    // validated JSON, so a short backward scan is sufficient.
                    let preceded_by_colon = bytes[..i]
                        .iter()
                        .rev()
                        .find(|b| !b.is_ascii_whitespace())
                        .is_some_and(|&b| b == b':');

                    if !preceded_by_colon {
                        // This is a key — style the content
                        let start = i + 1;
                        let mut j = start;
                        while j < bytes.len() {
                            if bytes[j] == b'\\' {
                                j += 2;
                                continue;
                            }
                            if bytes[j] == b'"' {
                                if start < j {
                                    collector.push(start, j, self.key);
                                }
                                collector.push(j, j + 1, self.quote_token);
                                in_string = false;
                                i = j;
                                break;
                            }
                            j += 1;
                        }
                    }
                }
                _ => {}
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn make_finder() -> JsonFinder {
        JsonFinder::new(
            Style::new().fg(Color::Yellow),
            Style::new().fg(Color::Blue),
            Style::new().fg(Color::Cyan),
            Style::new().fg(Color::Green),
            Style::new().fg(Color::Red),
            Style::new().fg(Color::Magenta),
        )
    }

    fn span_texts<'a>(input: &'a str, finder: &JsonFinder) -> Vec<&'a str> {
        let mut collector = Collector::new(0);
        finder.find_spans(input, &mut collector);
        collector.into_spans().iter().map(|s| &input[s.start..s.end]).collect()
    }

    #[test]
    fn simple_json_object() {
        let input = r#"{"name": "John", "age": 30}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"{"));
        assert!(texts.contains(&"}"));
        assert!(texts.contains(&"name"));
        assert!(texts.contains(&"age"));
        assert!(texts.contains(&","));
    }

    #[test]
    fn json_with_array() {
        let input = r#"{"items": [1, 2]}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"["));
        assert!(texts.contains(&"]"));
        assert!(texts.contains(&"items"));
    }

    #[test]
    fn nested_json() {
        let input = r#"{"a": {"b": 1}}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"a"));
        // Structural tokens are present
        assert!(!texts.is_empty());
    }

    #[test]
    fn json_with_escaped_quotes() {
        let input = r#"{"key": "val\"ue"}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"key"));
        // Should not panic or produce broken spans
        assert!(!texts.is_empty());
    }

    #[test]
    fn empty_json_object() {
        let input = "{}";
        let texts = span_texts(input, &make_finder());
        // Adjacent same-style brackets coalesce into one span
        assert_eq!(texts, ["{}"]);
    }

    #[test]
    fn empty_json_array() {
        let input = "[]";
        let texts = span_texts(input, &make_finder());
        assert_eq!(texts, ["[]"]);
    }

    #[test]
    fn not_json_no_match() {
        let mut collector = Collector::new(0);
        make_finder().find_spans("No jsons here!", &mut collector);
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn invalid_json_no_match() {
        let mut collector = Collector::new(0);
        make_finder().find_spans("{not valid json", &mut collector);
        assert!(collector.into_spans().is_empty());
    }

    #[test]
    fn key_after_non_string_value() {
        // The backward scan finds `,` before `"b"` regardless of value type.
        let input = r#"{"a": 1, "b": 2}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"a"));
        assert!(texts.contains(&"b"), "key after non-string value should be styled");
    }

    #[test]
    fn key_in_nested_object() {
        // The backward scan finds `{` before the inner key.
        let input = r#"{"a": {"b": 1}, "c": 2}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"a"));
        assert!(texts.contains(&"b"), "inner object key should be styled");
        assert!(texts.contains(&"c"), "key after nested object should be styled");
    }

    #[test]
    fn value_content_not_styled() {
        // The backward scan finds `:` before value strings — their content
        // should NOT appear in spans, only their quote tokens.
        let input = r#"{"a": "x", "b": {"c": "y"}}"#;
        let texts = span_texts(input, &make_finder());
        assert!(texts.contains(&"a"));
        assert!(texts.contains(&"b"));
        assert!(texts.contains(&"c"));
        assert!(!texts.contains(&"x"), "value content should not be styled");
        assert!(!texts.contains(&"y"), "nested value content should not be styled");
    }
}
