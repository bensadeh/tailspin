use std::fmt::Debug;
use thiserror::Error;
use HighlighterConfigNew::*;

#[derive(Copy, Clone)]
pub struct CliOpts {
    pub enable_numbers: bool,
    pub disable_numbers: bool,
    pub enable_uuids: bool,
    pub disable_uuids: bool,
    pub enable_quotes: bool,
    pub disable_quotes: bool,
    pub enable_ip_addresses: bool,
    pub disable_ip_addresses: bool,
    pub enable_dates: bool,
    pub disable_dates: bool,
    pub enable_paths: bool,
    pub disable_paths: bool,
    pub enable_urls: bool,
    pub disable_urls: bool,
    pub enable_pointers: bool,
    pub disable_pointers: bool,
    pub enable_processes: bool,
    pub disable_processes: bool,
    pub enable_key_value_pairs: bool,
    pub disable_key_value_pairs: bool,
}

pub enum HighlighterConfigNew {
    AllHighlightersEnabled,
    SomeHighlightersEnabled,
    SomeHighlightersDisabled,
    Mismatch,
}

pub struct HighlighterGroups {
    pub numbers: bool,
    pub uuids: bool,
    pub quotes: bool,
    pub ip_addresses: bool,
    pub dates: bool,
    pub paths: bool,
    pub urls: bool,
    pub pointers: bool,
    pub processes: bool,
    pub key_value_pairs: bool,
}

impl HighlighterGroups {
    const fn new_with_value(value: bool) -> Self {
        HighlighterGroups {
            numbers: value,
            uuids: value,
            quotes: value,
            ip_addresses: value,
            dates: value,
            paths: value,
            urls: value,
            pointers: value,
            processes: value,
            key_value_pairs: value,
        }
    }

    pub const fn all_enabled() -> Self {
        Self::new_with_value(true)
    }
}

pub const fn get_highlighter_groups(cli: CliOpts) -> Result<HighlighterGroups, ConfigError> {
    match determine_highlighter_type(cli) {
        AllHighlightersEnabled => Ok(HighlighterGroups::all_enabled()),
        SomeHighlightersEnabled => Ok(HighlighterGroups {
            numbers: cli.enable_numbers,
            uuids: cli.enable_uuids,
            quotes: cli.enable_quotes,
            ip_addresses: cli.enable_ip_addresses,
            dates: cli.enable_dates,
            paths: cli.enable_paths,
            urls: cli.enable_urls,
            pointers: cli.enable_pointers,
            processes: cli.enable_processes,
            key_value_pairs: cli.enable_key_value_pairs,
        }),
        SomeHighlightersDisabled => Ok(HighlighterGroups {
            numbers: !cli.disable_numbers,
            uuids: !cli.disable_uuids,
            quotes: !cli.disable_quotes,
            ip_addresses: !cli.disable_ip_addresses,
            dates: !cli.disable_dates,
            paths: !cli.disable_paths,
            urls: !cli.disable_urls,
            pointers: !cli.disable_pointers,
            processes: !cli.disable_processes,
            key_value_pairs: !cli.disable_key_value_pairs,
        }),
        Mismatch => Err(ConfigError::ConflictEnableDisable),
    }
}

pub const fn determine_highlighter_type(cli: CliOpts) -> HighlighterConfigNew {
    let some_enabled = cli.enable_numbers
        || cli.enable_uuids
        || cli.enable_quotes
        || cli.enable_ip_addresses
        || cli.enable_dates
        || cli.enable_paths
        || cli.enable_urls
        || cli.enable_pointers
        || cli.enable_processes
        || cli.enable_key_value_pairs;

    let some_disabled = cli.disable_numbers
        || cli.disable_uuids
        || cli.disable_quotes
        || cli.disable_ip_addresses
        || cli.disable_dates
        || cli.disable_paths
        || cli.disable_urls
        || cli.disable_pointers
        || cli.disable_processes
        || cli.disable_key_value_pairs;

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
