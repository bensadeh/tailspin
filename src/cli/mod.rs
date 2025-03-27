pub mod completions;
pub mod keywords;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum HighlighterGroup {
    Numbers,
    Urls,
    Pointers,
    Dates,
    Paths,
    Quotes,
    KeyValuePairs,
    Uuids,
    IpAddresses,
    Processes,
    Json,
}

#[derive(Parser)]
#[command(name = "tspin")]
#[command(author, version, about)]
pub struct Cli {
    /// Filepath
    #[clap(name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub file_path: Option<PathBuf>,

    /// Follow the contents of a file
    #[clap(short = 'f', long = "follow")]
    pub follow: bool,

    /// Start at the end of the file
    #[clap(short = 'e', long = "start-at-end")]
    pub start_at_end: bool,

    /// Print the output to stdout
    #[clap(short = 'p', long = "print")]
    pub to_stdout: bool,

    /// Provide a custom path to a configuration file
    #[clap(long = "config-path")]
    pub config_path: Option<String>,

    /// Capture the output (stdout) of a command and view it in `less`
    #[clap(short = 'c', long = "listen-command", conflicts_with = "follow")]
    pub listen_command: Option<String>,

    /// Highlight the provided words in red
    #[clap(long = "words-red", use_value_delimiter = true)]
    pub words_red: Vec<String>,

    /// Highlight the provided words in green
    #[clap(long = "words-green", use_value_delimiter = true)]
    pub words_green: Vec<String>,

    /// Highlight the provided words in yellow
    #[clap(long = "words-yellow", use_value_delimiter = true)]
    pub words_yellow: Vec<String>,

    /// Highlight the provided words in blue
    #[clap(long = "words-blue", use_value_delimiter = true)]
    pub words_blue: Vec<String>,

    /// Highlight the provided words in magenta
    #[clap(long = "words-magenta", use_value_delimiter = true)]
    pub words_magenta: Vec<String>,

    /// Highlight the provided words in cyan
    #[clap(long = "words-cyan", use_value_delimiter = true)]
    pub words_cyan: Vec<String>,

    /// Enable specific highlighters
    #[clap(long = "enable", value_enum, use_value_delimiter = true)]
    pub enable: Vec<HighlighterGroup>,

    /// Disable specific highlighters
    #[clap(long = "disable", value_enum, use_value_delimiter = true)]
    pub disable: Vec<HighlighterGroup>,

    /// Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)
    #[clap(long = "no-builtin-keywords")]
    pub no_builtin_keywords: bool,

    /// Print bash completions to stdout
    #[clap(long = "generate-bash-completions", hide = true)]
    pub generate_bash_completions: bool,

    /// Print fish completions to stdout
    #[clap(long = "generate-fish-completions", hide = true)]
    pub generate_fish_completions: bool,

    /// Print zsh completions to stdout
    #[clap(long = "generate-zsh-completions", hide = true)]
    pub generate_zsh_completions: bool,
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
