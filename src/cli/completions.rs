use crate::cli::Cli;
use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::exit;

pub fn generate_shell_completions_or_continue() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    if cli.generate_bash_completions {
        print_completions(Shell::Bash, &mut cmd);
        exit(0);
    }

    if cli.generate_fish_completions {
        print_completions(Shell::Fish, &mut cmd);
        exit(0);
    }

    if cli.generate_zsh_completions {
        print_completions(Shell::Zsh, &mut cmd);
        exit(0);
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
