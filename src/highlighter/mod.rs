pub mod config;

use crate::highlighter::config::HighlighterGroups;
use crate::theme::Theme;
use inlet_manifold::{Error, Highlighter};

fn get_highlighter(highlighter_groups: HighlighterGroups, theme: Theme) -> Result<Highlighter, Error> {
    let mut builder = Highlighter::builder();

    if highlighter_groups.numbers {
        builder.with_number_highlighter(theme.number);
    }

    builder.build()
}
