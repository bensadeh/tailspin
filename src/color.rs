use crate::theme::Style;
use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

pub const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Default, Copy)]
pub enum Fg {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Magenta,
    Cyan,
    #[default]
    None,
}

#[derive(Debug, Clone, Default)]
pub enum Bg {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    #[default]
    None,
}

pub fn to_ansi(style: &Style) -> String {
    let style_codes = [
        if style.bold { Some("1") } else { None },
        if style.italic { Some("3") } else { None },
        if style.underline { Some("4") } else { None },
        if style.faint { Some("2") } else { None },
    ];

    let fg_code = match style.fg {
        Fg::Red => Some("31"),
        Fg::Green => Some("32"),
        Fg::Blue => Some("34"),
        Fg::Yellow => Some("33"),
        Fg::White => Some("37"),
        Fg::Magenta => Some("35"),
        Fg::Cyan => Some("36"),
        Fg::None => None,
    };

    let bg_code = match style.bg {
        Bg::Red => Some("41"),
        Bg::Green => Some("42"),
        Bg::Blue => Some("44"),
        Bg::Yellow => Some("43"),
        Bg::White => Some("47"),
        Bg::None => None,
    };

    let codes = [
        style_codes[0],
        style_codes[1],
        style_codes[2],
        style_codes[3],
        fg_code,
        bg_code,
    ];

    let joined_codes = codes.iter().filter_map(|&code| code).collect::<Vec<&str>>().join(";");

    format!("\x1b[{}m", joined_codes)
}

impl FromStr for Fg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "red" => Ok(Fg::Red),
            "green" => Ok(Fg::Green),
            "blue" => Ok(Fg::Blue),
            "yellow" => Ok(Fg::Yellow),
            "magenta" => Ok(Fg::Magenta),
            "cyan" => Ok(Fg::Cyan),
            "white" => Ok(Fg::White),
            _ => Ok(Fg::None),
        }
    }
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fg::Red => write!(f, "\x1b[31m"),
            Fg::Green => write!(f, "\x1b[32m"),
            Fg::Yellow => write!(f, "\x1b[33m"),
            Fg::Blue => write!(f, "\x1b[34m"),
            Fg::Magenta => write!(f, "\x1b[35m"),
            Fg::Cyan => write!(f, "\x1b[36m"),
            Fg::White => write!(f, "\x1b[37m"),
            Fg::None => write!(f, "\x1b[0m"),
        }
    }
}

impl<'de> Deserialize<'de> for Fg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(FgVisitor)
    }
}

struct FgVisitor;

impl<'de> Visitor<'de> for FgVisitor {
    type Value = Fg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected a valid foreground color")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Fg, E> {
        let fg = v.parse().map_err(|_| E::custom("Parse error"))?;

        match fg {
            Fg::Red | Fg::Green | Fg::Blue | Fg::Yellow | Fg::Magenta | Fg::Cyan | Fg::White => Ok(fg),
            _ => {
                colored_panic("Invalid foreground color: ", v);
            }
        }
    }
}

impl FromStr for Bg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "red" => Ok(Bg::Red),
            "green" => Ok(Bg::Green),
            "blue" => Ok(Bg::Blue),
            "yellow" => Ok(Bg::Yellow),
            "white" => Ok(Bg::White),
            _ => Ok(Bg::None),
        }
    }
}

impl<'de> Deserialize<'de> for Bg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(BgVisitor)
    }
}

struct BgVisitor;

impl<'de> Visitor<'de> for BgVisitor {
    type Value = Bg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected a valid background color")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Bg, E> {
        let bg = v
            .parse()
            .map_err(|_| colored_panic("Parse error", &format!("Invalid background color: {}", v)))?;

        match bg {
            Bg::Red | Bg::Green | Bg::Blue | Bg::Yellow | Bg::White => Ok(bg),
            _ => {
                colored_panic("Invalid background color: ", v);
            }
        }
    }
}

fn colored_panic(message: &str, invalid_value: &str) -> ! {
    let color_yellow: &str = "\x1b[33m";
    let color_reset: &str = "\x1b[0m";

    let colored_message = format!("{}{}{}{}", message, color_yellow, invalid_value, color_reset,);
    eprintln!("{}", colored_message);
    std::process::exit(1);
}
