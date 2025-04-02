use crate::cli::Arguments;
use clap::{Command, CommandFactory};
use clap_complete::{Generator, Shell, generate};
use std::io;
use std::process::exit;

pub fn generate_shell_completions_and_exit_or_continue(cli: &Arguments) {
    let mut cmd = Arguments::command();

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

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
