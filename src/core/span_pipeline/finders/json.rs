use serde_json::Value;

use crate::style::Style;

use super::super::span::{Collector, Finder};

#[derive(Debug)]
pub struct JsonFinder {
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
        // For the span pipeline prototype, we do a simpler approach:
        // just validate it's JSON and style the structural characters in-place.
        if serde_json::from_str::<Value>(input).is_err() {
            return;
        }

        let bytes = input.as_bytes();
        let mut in_string = false;
        let mut escape_next = false;
        let mut after_colon = false;

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
                    after_colon = false;
                }
                i += 1;
                continue;
            }

            match b {
                b'{' | b'}' => collector.push(i, i + 1, self.curly_bracket),
                b'[' | b']' => collector.push(i, i + 1, self.square_bracket),
                b',' => collector.push(i, i + 1, self.comma),
                b':' => {
                    collector.push(i, i + 1, self.colon);
                    after_colon = true;
                }
                b'"' => {
                    collector.push(i, i + 1, self.quote_token);
                    in_string = true;
                    // Find the end of this string to determine if it's a key
                    if !after_colon {
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
