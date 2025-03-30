use crate::{
    Color, DateTimeConfig, IpV4Config, IpV6Config, JsonConfig, KeyValueConfig, NumberConfig, PointerConfig,
    QuotesConfig, Style, UnixPathConfig, UnixProcessConfig, UrlConfig, UuidConfig,
};

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
