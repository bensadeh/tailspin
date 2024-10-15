use crate::cli::HighlighterGroup;
use std::fmt::Debug;
use thiserror::Error;
use HighlighterConfigNew::{AllHighlightersEnabled, Mismatch, SomeHighlightersDisabled, SomeHighlightersEnabled};

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
    pub json: bool,
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
            json: value,
        }
    }

    pub const fn all_enabled() -> Self {
        Self::new_with_value(true)
    }
}

pub fn get_highlighter_groups(
    enabled: &[HighlighterGroup],
    disabled: &[HighlighterGroup],
) -> Result<HighlighterGroups, ConfigError> {
    match determine_highlighter_type_new(enabled, disabled) {
        AllHighlightersEnabled => Ok(HighlighterGroups::all_enabled()),
        SomeHighlightersEnabled => Ok(HighlighterGroups {
            numbers: enabled.contains(&HighlighterGroup::Numbers),
            uuids: enabled.contains(&HighlighterGroup::Uuids),
            quotes: enabled.contains(&HighlighterGroup::Quotes),
            ip_addresses: enabled.contains(&HighlighterGroup::IpAddresses),
            dates: enabled.contains(&HighlighterGroup::Dates),
            paths: enabled.contains(&HighlighterGroup::Paths),
            urls: enabled.contains(&HighlighterGroup::Urls),
            pointers: enabled.contains(&HighlighterGroup::Pointers),
            processes: enabled.contains(&HighlighterGroup::Processes),
            key_value_pairs: enabled.contains(&HighlighterGroup::KeyValuePairs),
            json: enabled.contains(&HighlighterGroup::Json),
        }),
        SomeHighlightersDisabled => Ok(HighlighterGroups {
            numbers: !disabled.contains(&HighlighterGroup::Numbers),
            uuids: !disabled.contains(&HighlighterGroup::Uuids),
            quotes: !disabled.contains(&HighlighterGroup::Quotes),
            ip_addresses: !disabled.contains(&HighlighterGroup::IpAddresses),
            dates: !disabled.contains(&HighlighterGroup::Dates),
            paths: !disabled.contains(&HighlighterGroup::Paths),
            urls: !disabled.contains(&HighlighterGroup::Urls),
            pointers: !disabled.contains(&HighlighterGroup::Pointers),
            processes: !disabled.contains(&HighlighterGroup::Processes),
            key_value_pairs: !disabled.contains(&HighlighterGroup::KeyValuePairs),
            json: !disabled.contains(&HighlighterGroup::Json),
        }),
        Mismatch => Err(ConfigError::ConflictEnableDisable),
    }
}

pub const fn determine_highlighter_type_new(
    enabled: &[HighlighterGroup],
    disabled: &[HighlighterGroup],
) -> HighlighterConfigNew {
    let some_enabled = !enabled.is_empty();
    let some_disabled = !disabled.is_empty();

    let none_enabled = enabled.is_empty();
    let none_disabled = disabled.is_empty();

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
