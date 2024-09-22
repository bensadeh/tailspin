mod config;

use crate::highlighter::config::HighlighterConfig;
use crate::theme::ProcessedTheme;
use inlet_manifold::*;

fn get_highlighter(theme: ProcessedTheme, config: HighlighterConfig) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if config.uuid {
        builder.with_uuid_highlighter(theme.uuid_config);
    }

    builder.build()
}
