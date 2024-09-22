mod config_mapper;
mod toml;

use crate::highlighter::config_mapper::ProcessedTheme;
use crate::highlighter::toml::{TomlTheme, UuidToml};
use inlet_manifold::*;

pub struct NewTheme {
    pub number_config: Option<NumberConfig>,
    pub quote_config: Option<QuoteConfig>,
}

fn get_highlighter(theme: NewTheme) -> Result<Highlighter, Error> {
    let toml_theme = TomlTheme {
        uuid: Some(UuidToml {
            number: Some(Style {
                fg: Some(Color::BrightBlue),
                italic: true,
                ..Style::default()
            }),
            letter: None,
            dash: None,
        }),
    };

    let _processed_theme: ProcessedTheme = toml_theme.into();

    let mut builder = Highlighter::builder();

    if let Some(number_config) = theme.number_config {
        builder.with_number_highlighter(number_config);
    }

    if let Some(quote_config) = theme.quote_config {
        builder.with_quote_highlighter(quote_config);
    }

    builder.build()
}
