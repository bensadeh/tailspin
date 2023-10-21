use crate::color::Fg;
use crate::theme::{FilePath, Ip, KeyValue, Style, Uuid};

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
