use crate::cli::Arguments;
use clap::CommandFactory;
use clap_complete::generate;
use std::io;
use std::process::exit;

pub fn generate_shell_completions_and_exit_or_continue(cli: &Arguments) {
    if let Some(shell) = cli.completions {
        let mut cmd = Arguments::command();
        let name = cmd.get_name().to_string();

        generate(shell, &mut cmd, name, &mut io::stdout());
        exit(0);
    }
}
