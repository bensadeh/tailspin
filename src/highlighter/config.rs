use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use HighlighterConfigNew::*;

#[derive(Copy, Clone)]
pub struct CliOpts {
    pub disable_numbers: bool,
    pub disable_letters: bool,
    pub disable_symbols: bool,
    pub enable_numbers: bool,
    pub enable_letters: bool,
    pub enable_symbols: bool,
}

pub enum HighlighterConfigNew {
    AllHighlightersEnabled,
    AllHighlightersDisabled,
    SomeHighlightersEnabled,
    SomeHighlightersDisabled,
    Mismatch,
}

pub struct HighlightGroups {
    pub numbers: bool,
    pub letters: bool,
    pub symbols: bool,
}

impl HighlightGroups {
    fn new_with_value(value: bool) -> Self {
        HighlightGroups {
            numbers: value,
            letters: value,
            symbols: value,
        }
    }

    pub fn all_enabled() -> Self {
        Self::new_with_value(true)
    }

    pub fn all_disabled() -> Self {
        Self::new_with_value(false)
    }
}

pub fn try_get_highlight_groups(cli: CliOpts) -> Result<HighlightGroups, ConfigError> {
    match determine_highlighter_type(cli) {
        AllHighlightersEnabled => Ok(HighlightGroups::all_enabled()),
        AllHighlightersDisabled => Ok(HighlightGroups::all_disabled()),
        SomeHighlightersEnabled => Ok(HighlightGroups {
            numbers: cli.enable_numbers,
            letters: cli.enable_letters,
            symbols: cli.enable_symbols,
        }),
        SomeHighlightersDisabled => Ok(HighlightGroups {
            numbers: !cli.disable_numbers,
            letters: !cli.disable_letters,
            symbols: !cli.disable_symbols,
        }),
        Mismatch => Err(ConfigError::ConflictEnableDisable),
    }
}

pub fn determine_highlighter_type(cli: CliOpts) -> HighlighterConfigNew {
    let enable_any = cli.enable_numbers || cli.enable_letters || cli.enable_symbols;
    let disable_any = cli.disable_numbers || cli.disable_letters || cli.disable_symbols;
    let enable_all = cli.enable_numbers && cli.enable_letters && cli.enable_symbols;
    let disable_all = cli.disable_numbers && cli.disable_letters && cli.disable_symbols;

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
