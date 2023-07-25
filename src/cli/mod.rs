use clap::Parser;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[command(name = "spin")]
#[command(about = "A log file highlighter")]
pub struct Args {
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
}

impl Args {
    pub fn parse_args() -> Args {
        Args::parse()
    }
}
