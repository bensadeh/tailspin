use nu_ansi_term::{Color as NuColor, Style as NuStyle};
use serde::Deserialize;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy, Default, Deserialize)]
pub struct Style {
    #[serde(default)]
    pub fg: Option<Color>,
    #[serde(default)]
    pub bg: Option<Color>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub faint: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
}

impl Style {
    pub fn new() -> Style {
        Style::default()
    }

    pub const fn bold(&self) -> Style {
        Style { bold: true, ..*self }
    }

    pub const fn faint(&self) -> Style {
        Style { faint: true, ..*self }
    }

    pub const fn italic(&self) -> Style {
        Style { italic: true, ..*self }
    }

    pub const fn underline(&self) -> Style {
        Style {
            underline: true,
            ..*self
        }
    }

    pub const fn fg(&self, fg: Color) -> Style {
        Style { fg: Some(fg), ..*self }
    }

    pub const fn on(&self, bg: Color) -> Style {
        Style { bg: Some(bg), ..*self }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl From<&Color> for NuColor {
    fn from(color: &Color) -> Self {
        match color {
            Color::Default => NuColor::Default,
            Color::Black => NuColor::Black,
            Color::Red => NuColor::Red,
            Color::Green => NuColor::Green,
            Color::Yellow => NuColor::Yellow,
            Color::Blue => NuColor::Blue,
            Color::Magenta => NuColor::Magenta,
            Color::Cyan => NuColor::Cyan,
            Color::White => NuColor::White,
            Color::BrightBlack => NuColor::DarkGray,
            Color::BrightRed => NuColor::LightRed,
            Color::BrightGreen => NuColor::LightGreen,
            Color::BrightYellow => NuColor::LightYellow,
            Color::BrightBlue => NuColor::LightBlue,
            Color::BrightMagenta => NuColor::LightMagenta,
            Color::BrightCyan => NuColor::LightCyan,
            Color::BrightWhite => NuColor::LightGray,
        }
    }
}

impl From<Style> for NuStyle {
    fn from(style: Style) -> Self {
        let mut nu_style = NuStyle::new();

        if let Some(fg) = &style.fg {
            nu_style = nu_style.fg(NuColor::from(fg));
        }
        if let Some(bg) = &style.bg {
            nu_style = nu_style.on(NuColor::from(bg));
        }

        if style.bold {
            nu_style = nu_style.bold();
        }
        if style.faint {
            nu_style = nu_style.dimmed();
        }
        if style.italic {
            nu_style = nu_style.italic();
        }
        if style.underline {
            nu_style = nu_style.underline();
        }

        nu_style
    }
}
