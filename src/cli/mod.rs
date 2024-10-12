pub mod keywords;

use clap::{Command, CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::exit;

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
    /// Path to file or folder
    #[clap(name = "FILE", value_hint = clap::ValueHint::AnyPath)]
    pub file_or_folder_path: Option<String>,

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

    /// Suppress all output (for debugging and benchmarking)
    #[clap(long = "suppress-output", hide = true)]
    pub suppress_output: bool,

    /// Print completions to stdout
    #[clap(long = "hidden-generate-shell-completions", hide = true)]
    pub generate_shell_completions: Option<String>,
}

pub fn get_args_or_exit_early() -> Cli {
    let args = Cli::parse();

    if should_exit_early(&args) {
        exit(0);
    }

    args
}

fn should_exit_early(args: &Cli) -> bool {
    if args.generate_shell_completions.is_some() {
        print_completions_to_stdout();
        return true;
    }

    false
}

pub fn print_completions_to_stdout() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    if let Some(shell) = args.generate_shell_completions {
        match shell.as_str() {
            "bash" => print_completions(Shell::Bash, &mut cmd),
            "zsh" => print_completions(Shell::Zsh, &mut cmd),
            "fish" => print_completions(Shell::Fish, &mut cmd),
            _ => (),
        }
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
