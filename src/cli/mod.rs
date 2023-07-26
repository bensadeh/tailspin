use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Parser)]
#[command(name = "spin")]
#[command(author, version, about)]
pub struct Cli {
    /// Filepath
    #[clap(name = "FILE")]
    pub file_path: Option<String>,

    /// Follow (tail) the contents of the file
    #[clap(short = 'f', long = "follow")]
    pub follow: bool,

    /// Print the output to stdout
    #[clap(short = 'p', long = "print", conflicts_with = "follow")]
    pub to_stdout: bool,

    /// Path to a custom configuration file
    #[clap(short = 'c', long = "config-path")]
    pub config_path: Option<String>,

    /// Tails the output of the provided command
    #[clap(short = 't', long = "tail-command")]
    pub tail_command: Option<String>,

    /// Generate a new configuration file
    #[clap(long = "generate-config")]
    pub generate_config: bool,

    /// Print the default configuration
    #[clap(long = "show-default-config", conflicts_with = "generate_config")]
    pub show_default_config: bool,

    /// Print zsh completions to stdout
    #[clap(long = "generate-completions", hide = true)]
    pub generate_completions: Option<String>,

    /// Print man page to stdout
    #[clap(long = "generate-man-page", hide = true)]
    pub generate_man_page: bool,
}

pub fn get_args() -> Cli {
    Cli::parse()
}

pub fn generate_completions() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    if let Some(shell) = args.generate_completions {
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
