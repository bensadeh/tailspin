use clap::builder::{Styles, styling};
use styling::AnsiColor;

pub const fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Magenta.on_default().bold())
        .usage(AnsiColor::Green.on_default().bold())
        .literal(AnsiColor::Blue.on_default().bold())
        .placeholder(AnsiColor::Yellow.on_default())
}
