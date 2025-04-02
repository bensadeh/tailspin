mod completions;
mod keywords;

use crate::cli::completions::generate_shell_completions_and_exit_or_continue;
use crate::cli::keywords::get_keywords_from_cli;
use crate::config::{Input, Output, get_io_config};
use crate::highlighter_builder;
use crate::highlighter_builder::builtins::get_builtin_keywords;
use crate::highlighter_builder::groups;
use crate::theme::reader;
use clap::{Parser, ValueEnum};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use tailspin::Highlighter;

#[derive(Parser)]
#[command(name = "tspin")]
#[command(author, version, about)]
pub struct Arguments {
    /// Filepath
    #[clap(name = "FILE", value_hint = clap::ValueHint::FilePath)]
    pub file_path: Option<PathBuf>,

    /// Follow the contents of a file
    #[clap(short = 'f', long = "follow")]
    pub follow: bool,

    /// Print the output to stdout
    #[clap(short = 'p', long = "print")]
    pub to_stdout: bool,

    /// Provide a custom path to a configuration file
    #[clap(long = "config-path")]
    pub config_path: Option<PathBuf>,

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
    pub enabled_highlighters: Vec<HighlighterGroup>,

    /// Disable specific highlighters
    #[clap(long = "disable", value_enum, use_value_delimiter = true)]
    pub disabled_highlighters: Vec<HighlighterGroup>,

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

pub struct FullConfig {
    pub input: Input,
    pub output: Output,
    pub highlighter: Highlighter,
}

pub fn get_config() -> Result<FullConfig> {
    let cli = Arguments::try_parse().into_diagnostic()?;
    generate_shell_completions_and_exit_or_continue(&cli);

    let io_config = get_io_config(&cli)?;
    let highlighter_groups = groups::get_highlighter_groups(&cli.enabled_highlighters, &cli.disabled_highlighters)?;

    let theme = reader::parse_theme(&cli.config_path)?;
    let keywords_builtin = get_builtin_keywords(cli.no_builtin_keywords);
    let keywords_from_toml = theme.keywords.clone();
    let keywords_from_cli = get_keywords_from_cli(&cli);

    let keywords = vec![]
        .into_iter()
        .chain(keywords_builtin)
        .chain(keywords_from_toml)
        .chain(keywords_from_cli)
        .collect();

    let highlighter = highlighter_builder::get_highlighter(highlighter_groups, theme, keywords)?;

    Ok(FullConfig {
        input: io_config.input,
        output: io_config.output,
        highlighter,
    })
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Arguments::command().debug_assert()
}
