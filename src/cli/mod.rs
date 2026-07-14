mod builtins;
mod completions;
mod default_theme;
mod highlighter;
pub(crate) mod keywords;
#[cfg(test)]
mod parity;
pub(crate) mod resolution;
mod styles;

use crate::cli::completions::generate_shell_completions_and_exit_or_continue;
use crate::cli::highlighter::build_highlighter;
use crate::cli::resolution::{BaseSet, resolve_extras};
use crate::cli::styles::{
    get_styles, help_with_env, help_with_possible_values, help_with_possible_values_and_env, help_with_value_list,
};
use crate::io::routing::{self, IoArgs, Source, Target};
use crate::theme::reader;
use anyhow::Result;
use clap::builder::StyledStr;
use clap::{ArgAction, Parser, ValueEnum};
use nu_ansi_term::Style;
use std::error::Error;
use std::io::{IsTerminal, stdin};
use std::path::PathBuf;
use tailspin::Highlighter;
use tailspin::style::Color;

const THEME_ENV: &str = "TAILSPIN_THEME";
const EXTRAS_ENV: &str = "TAILSPIN_EXTRAS";
const PAGER_ENV: &str = "TAILSPIN_PAGER";

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

    #[clap(long = "theme", value_name = "PATH", env = THEME_ENV, hide_env = true,
           help = help_with_env("Provide a custom path to a theme file", THEME_ENV))]
    pub theme: Option<PathBuf>,

    /// Run command and view the output in a pager
    #[clap(short = 'e', long = "exec", value_name = "COMMAND")]
    pub exec: Option<String>,

    #[arg(long = "highlight", value_name = "COLOR:WORDS", value_parser = parse_highlight, help = highlight_help())]
    pub color_word: Vec<(Color, Vec<String>)>,

    #[clap(long = "enable", value_enum, use_value_delimiter = true, hide_possible_values = true,
           help = help_with_possible_values::<Base>("Enable only the specified highlighters (disables the rest)"))]
    pub enabled: Vec<Base>,

    #[clap(long = "disable", value_enum, use_value_delimiter = true, hide_possible_values = true,
           help = help_with_possible_values::<Base>("Disable the specified highlighters (keeps the rest)"))]
    pub disabled: Vec<Base>,

    #[clap(long = "extras", value_enum, use_value_delimiter = true, env = EXTRAS_ENV,
           hide_env = true, hide_possible_values = true,
           help = help_with_possible_values_and_env::<Extra>("Enable extra highlighters (e.g., --extras ipv6)", EXTRAS_ENV))]
    pub extras: Vec<Extra>,

    #[clap(long = "pager", value_name = "COMMAND", env = PAGER_ENV, hide_env = true,
           help = help_with_env("Override the default pager command used by tspin. (e.g. `--pager=\"ov -f [FILE]\"`)", PAGER_ENV))]
    pub pager: Option<String>,

    #[clap(long = "completions", value_enum, value_name = "SHELL", hide_possible_values = true,
           help = help_with_possible_values::<clap_complete::Shell>("Print shell completions to stdout"))]
    pub completions: Option<clap_complete::Shell>,

    /// Print the default theme as a theme.toml to stdout
    #[clap(long = "generate-default-theme")]
    pub generate_default_theme: bool,
}

const HIGHLIGHT_COLORS: [&str; 16] = [
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright_black",
    "bright_red",
    "bright_green",
    "bright_yellow",
    "bright_blue",
    "bright_magenta",
    "bright_cyan",
    "bright_white",
];

fn highlight_help() -> StyledStr {
    help_with_value_list(
        "Highlights in the form color:word1,word2",
        "possible colors",
        &HIGHLIGHT_COLORS,
    )
}

fn parse_highlight(s: &str) -> Result<(Color, Vec<String>), Box<dyn Error + Send + Sync>> {
    let (color_str, words_str) = s
        .split_once(':')
        .ok_or_else(|| format!("Expected format COLOR:word1,word2,... found `{s}`"))?;

    let color = parse_color(color_str)?;

    let words = words_str.split(',').map(str::to_owned).collect();

    Ok((color, words))
}

/// The same `snake_case` color names `theme.toml` accepts, minus `default`.
fn parse_color(s: &str) -> Result<Color, String> {
    match s.to_lowercase().as_str() {
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        "bright_black" => Ok(Color::BrightBlack),
        "bright_red" => Ok(Color::BrightRed),
        "bright_green" => Ok(Color::BrightGreen),
        "bright_yellow" => Ok(Color::BrightYellow),
        "bright_blue" => Ok(Color::BrightBlue),
        "bright_magenta" => Ok(Color::BrightMagenta),
        "bright_cyan" => Ok(Color::BrightCyan),
        "bright_white" => Ok(Color::BrightWhite),
        other => Err(format!(
            "unknown color `{other}` (expected black, red, green, yellow, blue, magenta, cyan or white, optionally prefixed with bright_)"
        )),
    }
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
    Durations,
    Paths,
    Quotes,
    KeyValuePairs,
    Uuids,
    Ipv4,
    Processes,
    Json,
    Keywords,
}

pub struct FullConfig {
    pub source: Source,
    pub target: Target,
    pub highlighter: Highlighter,
}

pub fn get_config() -> Result<FullConfig> {
    let cli = Arguments::parse();

    generate_shell_completions_and_exit_or_continue(&cli);

    if cli.generate_default_theme {
        print!("{}", default_theme::default_theme_toml());
        std::process::exit(0);
    }

    let std_in_has_data = !stdin().is_terminal();
    if cli.file_path.is_none() && cli.exec.is_none() && !std_in_has_data {
        let style = Style::new().fg(nu_ansi_term::Color::Cyan);
        eprintln!("Missing filename ({} for help)", style.paint("tspin --help"));

        std::process::exit(0);
    }

    let (source, target) = routing::resolve(IoArgs {
        file_path: cli.file_path.clone(),
        exec: cli.exec.clone(),
        to_stdout: cli.to_stdout,
        follow: cli.follow,
        pager: cli.pager.clone(),
        std_in_has_data,
    })?;

    let base = BaseSet::resolve(&cli.enabled, &cli.disabled)?;
    let extras = resolve_extras(&cli.extras);

    let theme = reader::parse_theme(cli.theme.as_ref())?;
    let highlighter = build_highlighter(&base, &extras, theme, &cli.color_word)?;

    Ok(FullConfig {
        source,
        target,
        highlighter,
    })
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Arguments::command().debug_assert();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_palette_color_parses() {
        for color in HIGHLIGHT_COLORS {
            let input = format!("{color}:foo");
            parse_highlight(&input).unwrap_or_else(|_| panic!("`{color}` should parse"));
        }
    }

    #[test]
    fn colors_parse_case_insensitively() {
        let (color, words) = parse_highlight("RED:foo,bar").unwrap();

        assert_eq!(color, Color::Red);
        assert_eq!(words, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn unknown_and_default_colors_are_rejected() {
        assert!(parse_highlight("pink:foo").is_err());
        assert!(parse_highlight("default:foo").is_err());
        assert!(parse_highlight("no-colon").is_err());
    }
}
