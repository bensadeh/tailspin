use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
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
    let enable_any = cli.enable_numbers || cli.enable_paths || cli.enable_urls;
    let disable_any = cli.disable_numbers || cli.disable_paths || cli.disable_urls;
    let enable_all = cli.enable_numbers && cli.enable_paths && cli.enable_urls;
    let disable_all = cli.disable_numbers && cli.disable_paths && cli.disable_urls;

    if enable_any && disable_any {
        return Mismatch;
    }

    if enable_all {
        return AllHighlightersEnabled;
    }

    if disable_all {
        return AllHighlightersDisabled;
    }

    if enable_any {
        return SomeHighlightersEnabled;
    }

    if disable_any {
        return SomeHighlightersDisabled;
    }

    Mismatch
}

pub enum ConfigError {
    ConflictEnableDisable,
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ConflictEnableDisable => write!(f, "ConflictEnableDisable"),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ConflictEnableDisable => {
                write!(f, "Cannot both enable and disable highlighters")
            }
        }
    }
}

impl Error for ConfigError {}
