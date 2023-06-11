use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
pub enum Style {
    Bold,
    Italic,
    Faint,
    #[default]
    None,
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

impl ToString for Fg {
    fn to_string(&self) -> String {
        match self {
            Fg::Red => "\x1b[31m",
            Fg::Green => "\x1b[32m",
            Fg::Yellow => "\x1b[33m",
            Fg::Blue => "\x1b[34m",
            Fg::Magenta => "\x1b[35m",
            Fg::Cyan => "\x1b[36m",
            Fg::White => "\x1b[37m",
            Fg::None => "\x1b[0m",
        }
        .to_string()
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
        v.parse().map_err(|_| E::custom("Parse error"))
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
        formatter.write_str("expected a valid foreground color")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Bg, E> {
        v.parse().map_err(|_| E::custom("Parse error"))
    }
}

impl FromStr for Style {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bold" => Ok(Style::Bold),
            "faint" => Ok(Style::Faint),
            "italic" => Ok(Style::Italic),
            _ => Ok(Style::None),
        }
    }
}

impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(StyleVisitor)
    }
}

struct StyleVisitor;

impl<'de> Visitor<'de> for StyleVisitor {
    type Value = Style;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expected a valid foreground color")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Style, E> {
        v.parse().map_err(|_| E::custom("Parse error"))
    }
}
