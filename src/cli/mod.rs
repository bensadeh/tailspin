use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::exit;

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

    /// Disable the highlighting of all builtin keyword groups (booleans, severity and REST)
    #[clap(long = "disable-builtin-keywords")]
    pub disable_keyword_builtins: bool,

    /// Disable the highlighting of booleans and nulls
    #[clap(long = "disable-booleans")]
    pub disable_booleans: bool,

    /// Disable the highlighting of severity levels
    #[clap(long = "disable-severity")]
    pub disable_severity: bool,

    /// Disable the highlighting of REST verbs
    #[clap(long = "disable-rest")]
    pub disable_rest: bool,

    /// Enable the highlighting of numbers
    #[clap(long = "enable-numbers")]
    pub enable_numbers: bool,

    /// Enable the highlighting of dates
    #[clap(long = "enable-dates")]
    pub enable_dates: bool,

    /// Enable the highlighting of URLs
    #[clap(long = "enable-urls")]
    pub enable_urls: bool,

    /// Enable the highlighting of paths
    #[clap(long = "enable-paths")]
    pub enable_paths: bool,

    /// Enable the highlighting of quotes
    #[clap(long = "enable-quotes")]
    pub enable_quotes: bool,

    /// Enable the highlighting of key value pairs
    #[clap(long = "enable-key-value-pairs")]
    pub enable_key_value_pairs: bool,

    /// Enable the highlighting of UUIDs
    #[clap(long = "enable-uuids")]
    pub enable_uuids: bool,

    /// Enable the highlighting of IP addresses
    #[clap(long = "enable-ip-addresses")]
    pub enable_ip_addresses: bool,

    /// Enable the highlighting of pointers
    #[clap(long = "enable-pointers")]
    pub enable_pointers: bool,

    /// Enable the highlighting of unix processes
    #[clap(long = "enable-processes")]
    pub enable_processes: bool,

    /// Enable the highlighting of JSON
    #[clap(long = "enable-json")]
    pub enable_json: bool,

    /// Disable the highlighting of numbers
    #[clap(long = "disable-numbers")]
    pub disable_numbers: bool,

    /// Disable the highlighting of dates
    #[clap(long = "disable-dates")]
    pub disable_dates: bool,

    /// Disable the highlighting of URLs
    #[clap(long = "disable-urls")]
    pub disable_urls: bool,

    /// Disable the highlighting of paths
    #[clap(long = "disable-paths")]
    pub disable_paths: bool,

    /// Disable the highlighting of quotes
    #[clap(long = "disable-quotes")]
    pub disable_quotes: bool,

    /// Disable the highlighting of key value pairs
    #[clap(long = "disable-key-value-pairs")]
    pub disable_key_value_pairs: bool,

    /// Disable the highlighting of UUIDs
    #[clap(long = "disable-uuids")]
    pub disable_uuids: bool,

    /// Disable the highlighting of IP addresses
    #[clap(long = "disable-ip-addresses")]
    pub disable_ip_addresses: bool,

    /// Disable the highlighting of pointers
    #[clap(long = "disable-pointers")]
    pub disable_pointers: bool,

    /// Disable the highlighting of unix processes
    #[clap(long = "disable-processes")]
    pub disable_processes: bool,

    /// Disable the highlighting of JSON
    #[clap(long = "disable-json")]
    pub disable_json: bool,

    /// Suppress all output (for debugging and benchmarking)
    #[clap(long = "hidden-suppress-output", hide = true)]
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
