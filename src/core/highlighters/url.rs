use crate::core::config::UrlConfig;
use crate::core::core::Highlight;
use nu_ansi_term::Style as NuStyle;
use regex::{Captures, Error, Regex};

pub struct UrlHighlighter {
    url_regex: Regex,
    query_params_regex: Regex,
    http: NuStyle,
    https: NuStyle,
    host: NuStyle,
    path: NuStyle,
    query_params_key: NuStyle,
    query_params_value: NuStyle,
    symbols: NuStyle,
}

impl UrlHighlighter {
    pub fn new(config: UrlConfig) -> Result<Self, Error> {
        let url_regex = Regex::new(
            r"(?P<protocol>http|https)(:)(//)(?P<host>[^:/\n\s]+)(?P<path>[/a-zA-Z0-9\-_.]*)?(?P<query>\?[^#\n ]*)?",
        )?;

        let query_params_regex = Regex::new(r"(?P<delimiter>[?&])(?P<key>[^=]*)(?P<equal>=)(?P<value>[^&]*)")?;

        Ok(Self {
            url_regex,
            query_params_regex,
            http: config.http.into(),
            https: config.https.into(),
            host: config.host.into(),
            path: config.path.into(),
            query_params_key: config.query_params_key.into(),
            query_params_value: config.query_params_value.into(),
            symbols: config.symbols.into(),
        })
    }
}

impl Highlight for UrlHighlighter {
    fn apply(&self, input: &str) -> String {
        let highlighted = self.url_regex.replace_all(input, |caps: &Captures<'_>| {
            let mut output = String::new();

            if let Some(protocol) = caps.name("protocol") {
                let style = match protocol.as_str() {
                    "http" => self.http,
                    "https" => self.https,
                    _ => NuStyle::default(),
                };
                output.push_str(&format!("{}://", style.paint(protocol.as_str())));
            }

            if let Some(host) = caps.name("host") {
                output.push_str(&format!("{}", self.host.paint(host.as_str())));
            }

            if let Some(path) = caps.name("path") {
                output.push_str(&format!("{}", self.path.paint(path.as_str())));
            }

            if let Some(query) = caps.name("query") {
                let query_highlighted =
                    self.query_params_regex
                        .replace_all(query.as_str(), |query_caps: &Captures<'_>| {
                            let delimiter = query_caps.name("delimiter").map_or("", |m| m.as_str());
                            let key = query_caps.name("key").map_or("", |m| m.as_str());
                            let equal = query_caps.name("equal").map_or("", |m| m.as_str());
                            let value = query_caps.name("value").map_or("", |m| m.as_str());
                            format!(
                                "{}{}{}{}",
                                self.symbols.paint(delimiter),
                                self.query_params_key.paint(key),
                                self.symbols.paint(equal),
                                self.query_params_value.paint(value)
                            )
                        });
                output.push_str(&format!("{}", query_highlighted));
            }

            output
        });

        highlighted.into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tests::escape_code_converter::ConvertEscapeCodes;
    use crate::{Color, Style};

    #[test]
    fn test_url_highlighter() {
        let highlighter = UrlHighlighter::new(UrlConfig {
            http: Style::new().fg(Color::Yellow),
            https: Style::new().fg(Color::White),
            host: Style::new().fg(Color::Green),
            path: Style::new().fg(Color::Blue),
            query_params_key: Style::new().fg(Color::Magenta),
            query_params_value: Style::new().fg(Color::Cyan),
            symbols: Style::new().fg(Color::Red),
        })
        .unwrap();

        let cases = vec![
            (
                "https://www.openai.com/docs/api?apikey=abc123",
                "[white]https[reset]://[green]www.openai.com[reset][blue]/docs/api[reset][red]?[reset][magenta]apikey[reset][red]=[reset][cyan]abc123[reset]",
            ),
            (
                "https://api.example.org/api/v1/users?name=JohnDoe",
                "[white]https[reset]://[green]api.example.org[reset][blue]/api/v1/users[reset][red]?[reset][magenta]name[reset][red]=[reset][cyan]JohnDoe[reset]",
            ),
            (
                "http://example.com/path/to/resource?param1=value1&param2=value2",
                "[yellow]http[reset]://[green]example.com[reset][blue]/path/to/resource[reset][red]?[reset][magenta]param1[reset][red]=[reset][cyan]value1[reset][red]&[reset][magenta]param2[reset][red]=[reset][cyan]value2[reset]",
            ),
        ];

        for (input, expected) in cases {
            let actual = highlighter.apply(input);
            assert_eq!(expected, actual.convert_escape_codes());
        }
    }
}
