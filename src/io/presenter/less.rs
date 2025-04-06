use crate::io::presenter::Present;
use miette::{Diagnostic, Result};
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

pub struct Less {
    path: PathBuf,
    follow: bool,
}

#[derive(Debug, Error, Diagnostic)]
pub enum LessError {
    #[error(transparent)]
    CtrlCError(#[from] ctrlc::Error),

    #[error("Failed to execute 'less' command")]
    #[diagnostic(code(less::command_spawn))]
    CommandSpawn(#[source] std::io::Error),

    #[error("The 'less' command exited with non-zero status: {0}")]
    #[diagnostic(code(less::non_zero_exit))]
    NonZeroExit(std::process::ExitStatus),
}

impl Less {
    pub const fn new(path: PathBuf, follow: bool) -> Self {
        Self { path, follow }
    }
}

impl Present for Less {
    fn present(&self) -> Result<()> {
        ctrlc::set_handler(|| {}).map_err(LessError::CtrlCError)?;

        let args = get_args(self.follow);
        let status = Command::new("less")
            .env("LESSSECURE", "1")
            .args(&args)
            .arg(&self.path)
            .status()
            .map_err(LessError::CommandSpawn)?;

        status.success().then_some(()).ok_or(LessError::NonZeroExit(status))?;

        Ok(())
    }
}

fn get_args(follow: bool) -> Vec<String> {
    let mut args = vec![
        "--ignore-case".to_string(),
        "--RAW-CONTROL-CHARS".to_string(),
        "--".to_string(), // End of option arguments
    ];

    if follow {
        args.insert(0, "+F".to_string());
    }

    args
}
