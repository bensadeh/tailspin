mod completions;
mod keywords;
mod styles;

use crate::cli::completions::generate_shell_completions_and_exit_or_continue;
use crate::cli::keywords::get_keywords_from_cli;
use crate::cli::styles::get_styles;
use crate::config::{Source, Target, get_io_config};
use crate::highlighter_builder;
use crate::highlighter_builder::builtins::get_builtin_keywords;
use crate::highlighter_builder::groups;
use crate::theme::reader;
use clap::{Parser, ValueEnum};
use miette::Result;
use nu_ansi_term::Style;
use std::error::Error;
use std::io::{IsTerminal, stdin};
use std::path::PathBuf;
use tailspin::Highlighter;

#[derive(Parser)]
#[command(
    name = "tspin",
    version,
    about,
    author,
    styles = get_styles(),
    max_term_width = 105,
)]
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

    /// Run command and view the output in a pager
    #[clap(short = 'e', long = "exec")]
    pub exec: Option<String>,

    /// Highlights in the form color:word1,word2
    ///
    /// [possible values: red, green, yellow, blue, magenta, cyan]
    #[arg(long = "highlight", value_parser = parse_highlight)]
    pub color_word: Vec<(KeywordColor, Vec<String>)>,

    /// Enable specific highlighters
    #[clap(long = "enable", value_enum, use_value_delimiter = true)]
    pub enabled_highlighters: Vec<HighlighterGroup>,

    /// Disable specific highlighters
    #[clap(long = "disable", value_enum, use_value_delimiter = true)]
    pub disabled_highlighters: Vec<HighlighterGroup>,

    /// Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)
    #[clap(long = "disable-builtin-keywords")]
    pub disable_builtin_keywords: bool,

    /// Override the default pager command used by tspin.
    ///
    /// The provided command must include the placeholder `[FILE]`, which will be replaced with the file path internally.
    ///
    /// Example:
    ///   tspin --pager="ov -f [FILE]" logfile.txt
    #[clap(long = "pager", env = "TAILSPIN_PAGER")]
    pub pager: Option<String>,

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

fn parse_highlight(s: &str) -> Result<(KeywordColor, Vec<String>), Box<dyn Error + Send + Sync>> {
    let (color_str, words_str) = s
        .split_once(':')
        .ok_or_else(|| format!("Expected format COLOR:word1,word2,... found `{}`", s))?;

    let color = KeywordColor::from_str(color_str, true)?;

    let words = words_str.split(',').map(str::to_owned).collect();

    Ok((color, words))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum KeywordColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
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
    pub source: Source,
    pub target: Target,
    pub highlighter: Highlighter,
}

pub fn get_config() -> Result<FullConfig> {
    let cli = Arguments::parse();

    generate_shell_completions_and_exit_or_continue(&cli);

    let std_in_has_no_data = stdin().is_terminal();
    if cli.file_path.is_none() && cli.exec.is_none() && std_in_has_no_data {
        let style = Style::new().fg(nu_ansi_term::Color::Cyan);
        println!("Missing filename ({} for help)", style.paint("tspin --help"));

        std::process::exit(0);
    }

    let io_config = get_io_config(&cli)?;
    let highlighter_groups = groups::get_highlighter_groups(&cli.enabled_highlighters, &cli.disabled_highlighters)?;

    let theme = reader::parse_theme(&cli.config_path)?;
    let keywords_builtin = get_builtin_keywords(cli.disable_builtin_keywords);
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
        source: io_config.source,
        target: io_config.target,
        highlighter,
    })
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Arguments::command().debug_assert()
}
