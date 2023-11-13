use crate::color::{Bg, Fg};
use crate::theme::{Date, FilePath, Ip, KeyValue, Keyword, Number, Process, Quotes, Shorten, Style, Time, Url, Uuid};

impl Default for Uuid {
    fn default() -> Self {
        Uuid {
            segment: Style {
                fg: Fg::Blue,
                italic: true,
                ..Default::default()
            },
            separator: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for Ip {
    fn default() -> Self {
        Ip {
            segment: Style {
                fg: Fg::Blue,
                italic: true,
                ..Default::default()
            },
            separator: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key: Style {
                faint: true,
                ..Default::default()
            },
            separator: Style {
                fg: Fg::White,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for FilePath {
    fn default() -> Self {
        FilePath {
            segment: Style {
                fg: Fg::Green,
                italic: true,
                ..Default::default()
            },
            separator: Style {
                fg: Fg::Yellow,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Date {
            style: Style {
                fg: Fg::Magenta,
                ..Default::default()
            },
            shorten: None,
            disabled: false,
        }
    }
}

impl Default for Shorten {
    fn default() -> Self {
        Shorten {
            to: "␣".to_owned(),
            style: Style { ..Default::default() },
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            time: Style {
                fg: Fg::Blue,
                ..Default::default()
            },
            zone: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            shorten: None,
            disabled: false,
        }
    }
}

impl Default for Process {
    fn default() -> Self {
        Process {
            name: Style {
                fg: Fg::Green,
                ..Default::default()
            },
            id: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            separator: Style {
                fg: Fg::Yellow,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Number {
            style: Style {
                fg: Fg::Cyan,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

impl Default for Quotes {
    fn default() -> Self {
        Quotes {
            style: Style {
                fg: Fg::Yellow,
                ..Default::default()
            },
            token: '"',
            disabled: false,
        }
    }
}

impl Default for Url {
    fn default() -> Self {
        Url {
            http: Style {
                faint: true,
                ..Default::default()
            },
            https: Style {
                bold: true,
                ..Default::default()
            },
            host: Style {
                fg: Fg::Blue,
                faint: true,
                ..Default::default()
            },
            path: Style {
                fg: Fg::Blue,
                ..Default::default()
            },
            query_params_key: Style {
                fg: Fg::Magenta,
                ..Default::default()
            },
            query_params_value: Style {
                fg: Fg::Cyan,
                ..Default::default()
            },
            symbols: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            disabled: false,
        }
    }
}

pub fn get_severity_keywords() -> Vec<Keyword> {
    vec![
        Keyword {
            words: vec!["ERROR".to_string()],
            style: Style {
                fg: Fg::Red,
                ..Default::default()
            },
            border: false,
        },
        Keyword {
            words: vec!["WARN".to_string(), "WARNING".to_string()],
            style: Style {
                fg: Fg::Yellow,
                ..Default::default()
            },
            border: false,
        },
        Keyword {
            words: vec!["INFO".to_string()],
            style: Style {
                fg: Fg::White,
                ..Default::default()
            },
            border: false,
        },
        Keyword {
            words: vec!["DEBUG".to_string(), "SUCCESS".to_string()],
            style: Style {
                fg: Fg::Green,
                ..Default::default()
            },
            border: false,
        },
        Keyword {
            words: vec!["TRACE".to_string()],
            style: Style {
                faint: true,
                ..Default::default()
            },
            border: false,
        },
    ]
}

pub fn get_rest_keywords() -> Vec<Keyword> {
    vec![
        Keyword {
            words: vec!["GET".to_string(), "HEAD".to_string()],
            style: Style {
                fg: Fg::Black,
                bg: Bg::Green,
                ..Default::default()
            },
            border: true,
        },
        Keyword {
            words: vec!["POST".to_string()],
            style: Style {
                fg: Fg::Black,
                bg: Bg::Yellow,
                ..Default::default()
            },
            border: true,
        },
        Keyword {
            words: vec!["PUT".to_string(), "PATCH".to_string()],
            style: Style {
                fg: Fg::Black,
                bg: Bg::Magenta,
                ..Default::default()
            },
            border: true,
        },
        Keyword {
            words: vec!["DELETE".to_string()],
            style: Style {
                fg: Fg::Black,
                bg: Bg::Red,
                ..Default::default()
            },
            border: true,
        },
    ]
}

pub fn get_boolean_keywords() -> Vec<Keyword> {
    vec![Keyword {
        words: vec!["null".to_string(), "true".to_string(), "false".to_string()],
        style: Style {
            fg: Fg::Red,
            italic: true,
            ..Default::default()
        },
        border: false,
    }]
}
