use crate::style::{Color, Style};

/// Configuration for highlighting numeric values.
pub struct NumberConfig {
    /// Style applied to numbers.
    pub style: Style,
}

/// Configuration for highlighting UUIDs.
pub struct UuidConfig {
    /// Style applied to numeric characters.
    pub number: Style,
    /// Style applied to alphabetic characters.
    pub letter: Style,
    /// Style applied to dashes (`-`).
    pub dash: Style,
}

/// Configuration for highlighting key-value pairs.
pub struct KeyValueConfig {
    /// Style for the key portion.
    pub key: Style,
    /// Style for the separator between key and value.
    pub separator: Style,
}

/// Configuration for highlighting date-time strings.
#[derive(Clone, Copy)]
pub struct DateTimeConfig {
    /// Style for dates.
    pub date: Style,
    /// Style for times.
    pub time: Style,
    /// Style for timezone indicators.
    pub zone: Style,
    /// Style for separators.
    pub separator: Style,
}

/// Configuration for highlighting IPv4 addresses.
pub struct IpV4Config {
    /// Style for numeric segments.
    pub number: Style,
    /// Style for dot separators (`.`).
    pub separator: Style,
}

/// Configuration for highlighting IPv6 addresses.
pub struct IpV6Config {
    /// Style for numeric characters.
    pub number: Style,
    /// Style for alphabetic characters.
    pub letter: Style,
    /// Style for colon separators (`:`).
    pub separator: Style,
}

/// Configuration for highlighting URLs.
pub struct UrlConfig {
    /// Style for "http" scheme.
    pub http: Style,
    /// Style for "https" scheme.
    pub https: Style,
    /// Style for the hostname.
    pub host: Style,
    /// Style for URL paths.
    pub path: Style,
    /// Style for query parameter keys.
    pub query_params_key: Style,
    /// Style for query parameter values.
    pub query_params_value: Style,
    /// Style for URL symbols (e.g., `/`, `:`, `?`).
    pub symbols: Style,
}

/// Configuration for highlighting Unix file paths.
pub struct UnixPathConfig {
    /// Style for path segments.
    pub segment: Style,
    /// Style for path separators (`/`).
    pub separator: Style,
}

/// Configuration for highlighting memory pointers.
pub struct PointerConfig {
    /// Style for numeric digits.
    pub number: Style,
    /// Style for alphabetic characters.
    pub letter: Style,
    /// Style for pointer separators.
    pub separator: Style,
    /// Token used to separate segments.
    pub separator_token: char,
    /// Style for the `x` character in pointers.
    pub x: Style,
}

/// Configuration for highlighting Unix processes.
pub struct UnixProcessConfig {
    /// Style for process name.
    pub name: Style,
    /// Style for process ID.
    pub id: Style,
    /// Style for surrounding brackets.
    pub bracket: Style,
}

/// Configuration for highlighting JSON structures.
pub struct JsonConfig {
    /// Style for JSON keys.
    pub key: Style,
    /// Style for quotation marks (`"`).
    pub quote_token: Style,
    /// Style for curly brackets (`{}`).
    pub curly_bracket: Style,
    /// Style for square brackets (`[]`).
    pub square_bracket: Style,
    /// Style for commas (`,`).
    pub comma: Style,
    /// Style for colons (`:`).
    pub colon: Style,
}

/// Configuration for highlighting quoted text.
pub struct QuotesConfig {
    /// Character used as quote delimiter.
    pub quotes_token: char,
    /// Style applied to quoted text.
    pub style: Style,
}

/// Configuration for highlighting custom keywords.
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub struct KeywordConfig {
    /// List of keywords to highlight.
    pub words: Vec<String>,
    /// Style to apply to the keywords.
    pub style: Style,
}

/// Configuration for highlighting custom regex patterns.
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub struct RegexConfig {
    /// Regex pattern for matching text.
    pub regex: String,
    /// Style applied to regex matches.
    pub style: Style,
}

impl Default for NumberConfig {
    fn default() -> Self {
        NumberConfig {
            style: Style::new().fg(Color::Cyan),
        }
    }
}

impl Default for UuidConfig {
    fn default() -> Self {
        UuidConfig {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            dash: Style::new().fg(Color::Red),
        }
    }
}

impl Default for KeyValueConfig {
    fn default() -> Self {
        KeyValueConfig {
            key: Style::new().faint(),
            separator: Style::new().fg(Color::White),
        }
    }
}

impl Default for DateTimeConfig {
    fn default() -> Self {
        DateTimeConfig {
            date: Style::new().fg(Color::Magenta),
            time: Style::new().fg(Color::Blue),
            zone: Style::new().fg(Color::Red),
            separator: Style::new().faint(),
        }
    }
}

impl Default for IpV4Config {
    fn default() -> Self {
        IpV4Config {
            number: Style::new().fg(Color::Blue).italic(),
            separator: Style::new().fg(Color::Red),
        }
    }
}

impl Default for IpV6Config {
    fn default() -> Self {
        IpV6Config {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().fg(Color::Red),
        }
    }
}

impl Default for UrlConfig {
    fn default() -> Self {
        UrlConfig {
            http: Style::new().fg(Color::Red).faint(),
            https: Style::new().fg(Color::Green).faint(),
            host: Style::new().fg(Color::Blue).faint(),
            path: Style::new().fg(Color::Blue),
            query_params_key: Style::new().fg(Color::Magenta),
            query_params_value: Style::new().fg(Color::Cyan),
            symbols: Style::new().fg(Color::Red),
        }
    }
}

impl Default for UnixPathConfig {
    fn default() -> Self {
        UnixPathConfig {
            segment: Style::new().fg(Color::Green),
            separator: Style::new().fg(Color::Yellow),
        }
    }
}

impl Default for PointerConfig {
    fn default() -> Self {
        PointerConfig {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().faint(),
            separator_token: '•',
            x: Style::new().fg(Color::Red),
        }
    }
}

impl Default for UnixProcessConfig {
    fn default() -> Self {
        UnixProcessConfig {
            name: Style::new().fg(Color::Yellow),
            id: Style::new().fg(Color::Cyan),
            bracket: Style::new().fg(Color::Red),
        }
    }
}

impl Default for JsonConfig {
    fn default() -> Self {
        JsonConfig {
            key: Style::new().fg(Color::Yellow),
            quote_token: Style::new().fg(Color::Yellow).faint(),
            curly_bracket: Style::new().faint(),
            square_bracket: Style::new().faint(),
            comma: Style::new().faint(),
            colon: Style::new().faint(),
        }
    }
}

impl Default for QuotesConfig {
    fn default() -> Self {
        QuotesConfig {
            quotes_token: '"',
            style: Style::new().fg(Color::Yellow),
        }
    }
}
