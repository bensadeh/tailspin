use std::fmt::Debug;
use thiserror::Error;
use HighlighterConfigNew::*;

#[derive(Copy, Clone)]
pub struct CliOpts {
    pub disable_numbers: bool,
    pub disable_paths: bool,
    pub disable_urls: bool,
    pub enable_numbers: bool,
    pub enable_paths: bool,
    pub enable_urls: bool,
}

pub enum HighlighterConfigNew {
    AllHighlightersEnabled,
    AllHighlightersDisabled,
    SomeHighlightersEnabled,
    SomeHighlightersDisabled,
    Mismatch,
}

pub struct HighlighterGroups {
    pub numbers: bool,
    pub paths: bool,
    pub urls: bool,
}

impl HighlighterGroups {
    const fn new_with_value(value: bool) -> Self {
        HighlighterGroups {
            numbers: value,
            paths: value,
            urls: value,
        }
    }

    pub const fn all_enabled() -> Self {
        Self::new_with_value(true)
    }

    pub const fn all_disabled() -> Self {
        Self::new_with_value(false)
    }
}

pub const fn try_get_highlight_groups(cli: CliOpts) -> Result<HighlighterGroups, ConfigError> {
    match determine_highlighter_type(cli) {
        AllHighlightersEnabled => Ok(HighlighterGroups::all_enabled()),
        AllHighlightersDisabled => Ok(HighlighterGroups::all_disabled()),
        SomeHighlightersEnabled => Ok(HighlighterGroups {
            numbers: cli.enable_numbers,
            paths: cli.enable_paths,
            urls: cli.enable_urls,
        }),
        SomeHighlightersDisabled => Ok(HighlighterGroups {
            numbers: !cli.disable_numbers,
            paths: !cli.disable_paths,
            urls: !cli.disable_urls,
        }),
        Mismatch => Err(ConfigError::ConflictEnableDisable),
    }
}

pub const fn determine_highlighter_type(cli: CliOpts) -> HighlighterConfigNew {
    let some_enabled = cli.enable_numbers || cli.enable_paths || cli.enable_urls;
    let some_disabled = cli.disable_numbers || cli.disable_paths || cli.disable_urls;
    let all_enabled = cli.enable_numbers && cli.enable_paths && cli.enable_urls;
    let all_disabled = cli.disable_numbers && cli.disable_paths && cli.disable_urls;

    let none_enabled = !all_enabled;
    let none_disabled = !all_disabled;
    let only_some_enabled = some_enabled && none_disabled;
    let only_some_disabled = some_disabled && none_enabled;

    if none_disabled && none_enabled {
        return AllHighlightersEnabled;
    }

    if only_some_enabled {
        return SomeHighlightersEnabled;
    }

    if only_some_disabled {
        return SomeHighlightersDisabled;
    }

    Mismatch
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("cannot both enable and disable highlighters")]
    ConflictEnableDisable,
}
