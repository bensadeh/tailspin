use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct HighlighterConfig {
    pub numbers: bool,
    pub uuid: bool,
    pub letters: bool,
    pub symbols: bool,
}

impl HighlighterConfig {
    fn new_all(state: bool) -> Self {
        HighlighterConfig {
            numbers: state,
            uuid: state,
            letters: state,
            symbols: state,
        }
    }

    fn enable_all() -> Self {
        Self::new_all(true)
    }

    fn disable_all() -> Self {
        Self::new_all(false)
    }
}

pub struct CliOpts {
    pub disable_numbers: bool,
    pub disable_letters: bool,
    pub disable_symbols: bool,
    pub enable_numbers: bool,
    pub enable_letters: bool,
    pub enable_symbols: bool,
    // more items with time
}

pub fn get_config(cli: CliOpts) -> Result<HighlighterConfig, ConfigError> {
    let any_enable = cli.enable_numbers || cli.enable_letters || cli.enable_symbols;
    let any_disable = cli.disable_numbers || cli.disable_letters || cli.disable_symbols;

    if any_enable && any_disable {
        return Err(ConfigError::ConflictEnableDisable);
    }

    let mut config = HighlighterConfig::enable_all();

    if any_enable {
        // Start with all items off
        config.numbers = false;
        config.letters = false;
        config.symbols = false;
        // Set specified items to true
        if cli.enable_numbers {
            config.numbers = true;
        }
        if cli.enable_letters {
            config.letters = true;
        }
        if cli.enable_symbols {
            config.symbols = true;
        }
    } else if any_disable {
        // Start with all items on (already set by default)
        // Set specified items to false
        if cli.disable_numbers {
            config.numbers = false;
        }
        if cli.disable_letters {
            config.letters = false;
        }
        if cli.disable_symbols {
            config.symbols = false;
        }
    }
    // If neither enable nor disable options are set, default config is used (all items are on)

    Ok(config)
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
                write!(f, "Cannot specify both enable and disable options for the same items")
            }
        }
    }
}

impl Error for ConfigError {}
