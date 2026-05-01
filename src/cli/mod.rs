mod completions;
mod keywords;
pub(crate) mod resolution;
mod styles;

use crate::cli::completions::generate_shell_completions_and_exit_or_continue;
use crate::cli::keywords::collect_keywords;
use crate::cli::resolution::{BaseSet, resolve_extras};
use crate::cli::styles::get_styles;
use crate::config::{Source, Target, get_io_config};
use crate::highlighter_builder::{build_highlighter, build_pipeline};
use crate::theme::reader;
use anyhow::Result;
use clap::{ArgAction, Parser, ValueEnum};
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
    max_term_width = 120,
    disable_help_flag = true,
    arg(clap::Arg::new("help")
        .short('h')
        .long("help")
        .help("Print help")
        .action(ArgAction::HelpShort)),
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
    pub enabled: Vec<Base>,

    /// Disable specific highlighters
    #[clap(long = "disable", value_enum, use_value_delimiter = true)]
    pub disabled: Vec<Base>,

    /// Enable extra highlighters (e.g., --extras ipv6)
    #[clap(long = "extras", value_enum, use_value_delimiter = true, env = "TAILSPIN_EXTRAS")]
    pub extras: Vec<Extra>,

    /// Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)
    #[clap(long = "disable-builtin-keywords")]
    pub disable_builtin_keywords: bool,

    /// Override the default pager command used by tspin. (e.g. `--pager="ov -f [FILE]"`)
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
        .ok_or_else(|| format!("Expected format COLOR:word1,word2,... found `{s}`"))?;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum Extra {
    Ipv6,
    JvmStackTrace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash)]
pub enum Base {
    Numbers,
    Urls,
    Emails,
    Pointers,
    Dates,
    Paths,
    Quotes,
    KeyValuePairs,
    Uuids,
    Ipv4,
    Processes,
    Json,
}

impl Base {
    pub const ALL: &[Base] = &[
        Self::Numbers,
        Self::Urls,
        Self::Emails,
        Self::Pointers,
        Self::Dates,
        Self::Paths,
        Self::Quotes,
        Self::KeyValuePairs,
        Self::Uuids,
        Self::Ipv4,
        Self::Processes,
        Self::Json,
    ];
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

    let base = BaseSet::resolve(&cli.enabled, &cli.disabled)?;
    let extras = resolve_extras(&cli.extras);

    let mut theme = reader::parse_theme(cli.config_path.as_ref())?;
    let keywords = collect_keywords(&cli, std::mem::take(&mut theme.keywords));

    let stages = build_pipeline(&base, &extras, theme, keywords);
    let highlighter = build_highlighter(stages)?;

    Ok(FullConfig {
        source: io_config.source,
        target: io_config.target,
        highlighter,
    })
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Arguments::command().debug_assert();
}
