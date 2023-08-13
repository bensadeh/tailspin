use crate::theme_io;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::exit;

#[derive(Parser)]
#[command(name = "spin")]
#[command(author, version, about)]
pub struct Cli {
    /// Path to file or folder
    #[clap(name = "FILE")]
    pub file_or_folder_path: Option<String>,

    /// Follow (tail) the contents of the file
    #[clap(short = 'f', long = "follow")]
    pub follow: bool,

    /// Start at the end of the file
    #[clap(short = 't', long = "tail")]
    pub tail: bool,

    /// Print the output to stdout
    #[clap(short = 'p', long = "print", conflicts_with = "follow")]
    pub to_stdout: bool,

    /// Path to a custom configuration file
    #[clap(short = 'c', long = "config-path")]
    pub config_path: Option<String>,

    /// Continuously listens to the stdout of the provided command and prevents interrupt events (Ctrl + C) from reaching the command
    #[clap(short = 'l', long = "follow-command", conflicts_with = "follow")]
    pub listen_command: Option<String>,

    /// Generate a new configuration file
    #[clap(long = "create-default-config")]
    pub create_default_config: bool,

    /// Print the default configuration
    #[clap(long = "show-default-config", conflicts_with = "create_default_config")]
    pub show_default_config: bool,

    /// Print completions to stdout
    #[clap(long = "z-generate-shell-completions", hide = true)]
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

    if args.create_default_config {
        theme_io::create_default_config();
        return true;
    }

    if args.show_default_config {
        let default_config = theme_io::default_theme();
        println!("{}", default_config);
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
