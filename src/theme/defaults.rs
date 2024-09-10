use crate::theme::processed::*;
use nu_ansi_term::{Color, Style};

impl Default for Uuid {
    fn default() -> Self {
        Uuid {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            dash: Style::new().fg(Color::Red),
            disabled: false,
        }
    }
}

impl Default for Pointer {
    fn default() -> Self {
        Pointer {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().dimmed(),
            separator_token: '•',
            x: Style::new().fg(Color::Red),
            disabled: false,
        }
    }
}

impl Default for Ip {
    fn default() -> Self {
        Ip {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().fg(Color::Red),
            disabled: false,
        }
    }
}

impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key: Style::new().dimmed(),
            separator: Style::new().fg(Color::White),
            disabled: false,
        }
    }
}

impl Default for FilePath {
    fn default() -> Self {
        FilePath {
            segment: Style::new().fg(Color::Green).italic(),
            separator: Style::new().fg(Color::Yellow),
            disabled: false,
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Date {
            number: Style::new().fg(Color::Magenta),
            separator: Style::new().fg(Color::Default).dimmed(),
            disabled: false,
        }
    }
}

impl Default for DateWord {
    fn default() -> Self {
        DateWord {
            day: Style::new().fg(Color::Magenta),
            month: Style::new().fg(Color::Magenta),
            number: Style::new().fg(Color::Magenta),
            disabled: false,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            time: Style::new().fg(Color::Blue),
            zone: Style::new().fg(Color::Red),
            separator: Style::new().fg(Color::Default).dimmed(),
            disabled: false,
        }
    }
}

impl Default for Process {
    fn default() -> Self {
        Process {
            name: Style::new().fg(Color::Yellow),
            id: Style::new().fg(Color::Cyan),
            separator: Style::new().fg(Color::Red),
            disabled: false,
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number {
            style: Style::new().fg(Color::Cyan),
            disabled: false,
        }
    }
}

impl Default for Quotes {
    fn default() -> Self {
        Quotes {
            style: Style::new().fg(Color::Yellow),
            token: '"',
            disabled: false,
        }
    }
}

impl Default for Url {
    fn default() -> Self {
        Url {
            http: Style::new().fg(Color::Red).dimmed(),
            https: Style::new().fg(Color::Green).dimmed(),
            host: Style::new().fg(Color::Blue).dimmed(),
            path: Style::new().fg(Color::Blue),
            query_params_key: Style::new().fg(Color::Magenta),
            query_params_value: Style::new().fg(Color::Cyan),
            symbols: Style::new().fg(Color::Red),
            disabled: false,
        }
    }
}

pub fn get_severity_keywords() -> Vec<Keyword> {
    vec![
        Keyword {
            words: vec!["ERROR".to_string()],
            style: Style::new().fg(Color::Red),
            border: false,
        },
        Keyword {
            words: vec!["WARN".to_string(), "WARNING".to_string()],
            style: Style::new().fg(Color::Yellow),
            border: false,
        },
        Keyword {
            words: vec!["INFO".to_string()],
            style: Style::new().fg(Color::White),
            border: false,
        },
        Keyword {
            words: vec!["DEBUG".to_string(), "SUCCESS".to_string()],
            style: Style::new().fg(Color::Green),
            border: false,
        },
        Keyword {
            words: vec!["TRACE".to_string()],
            style: Style::new().dimmed(),
            border: false,
        },
    ]
}

pub fn get_rest_keywords() -> Vec<Keyword> {
    vec![
        Keyword {
            words: vec!["GET".to_string(), "HEAD".to_string(), "PROPFIND".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Green),
            border: true,
        },
        Keyword {
            words: vec!["POST".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Yellow),
            border: true,
        },
        Keyword {
            words: vec!["PUT".to_string(), "PATCH".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Magenta),
            border: true,
        },
        Keyword {
            words: vec!["DELETE".to_string()],
            style: Style::new().fg(Color::Black).on(Color::Red),
            border: true,
        },
    ]
}

pub fn get_boolean_keywords() -> Vec<Keyword> {
    vec![Keyword {
        words: vec!["null".to_string(), "true".to_string(), "false".to_string()],
        style: Style::new().fg(Color::Red).italic(),
        border: false,
    }]
}
