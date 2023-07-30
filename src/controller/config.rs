use crate::cli::Cli;
use tokio::sync::oneshot::Sender;

// pub fn create_config(args: Cli) -> Result<Config, Error> {
//
//     let config = Config {
//         input: /* Fill in based on `args` */,
//         output: /* Fill in based on `args` */,
//         follow: /* Fill in based on `args` */,
//         reached_eof_tx: /* Fill in based on `args` */,
//     };
//
//     Ok(config)
// }

pub struct Error {
    exit_code: usize,
    message: String,
}

pub struct Config {
    input: Input,
    output: Output,
    follow: bool,
    reached_eof_tx: Option<Sender<()>>,
}

pub enum Input {
    FilePath(File),
    FolderPath(String),
    Stdin,
}

pub struct File {
    path: String,
    line_count: usize,
}

pub enum Output {
    TempFile,
    Stdout,
}
