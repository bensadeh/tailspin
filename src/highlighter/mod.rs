mod config;

use crate::highlighter::config::HighlightGroups;
use crate::theme::ProcessedTheme;
use inlet_manifold::{Error, Highlighter};

fn get_highlighter(theme: ProcessedTheme, config: HighlightGroups) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if config.letters {
        builder.with_uuid_highlighter(theme.uuid_config);
    }

    builder.build()
}
