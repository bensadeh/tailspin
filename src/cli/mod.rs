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
    #[clap(long = "create-default-config")]
    pub create_default_config: bool,

    /// Print the default configuration
    #[clap(long = "show-default-config", conflicts_with = "create_default_config")]
    pub show_default_config: bool,

    /// Print zsh completions to stdout
    #[clap(long = "z-generate", hide = true)]
    pub generate_completions_or_man_pages: Option<String>,
}

pub fn get_args() -> Cli {
    let a = Cli::parse();

    a
}

pub fn print_completions_or_man_pages_to_stdout() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    if let Some(shell) = args.generate_completions_or_man_pages {
        match shell.as_str() {
            "bash" => print_completions(Shell::Bash, &mut cmd),
            "zsh" => print_completions(Shell::Zsh, &mut cmd),
            "fish" => print_completions(Shell::Fish, &mut cmd),
            "man" => print_man_pages(cmd),
            _ => (),
        }
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn print_man_pages(cmd: Command) {
    let man = clap_mangen::Man::new(cmd);
    man.render(&mut io::stdout())
        .expect("Could not print man pages to stdout");
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
