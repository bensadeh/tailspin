use crate::io::presenter::Present;
use miette::{Diagnostic, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

pub struct Pager {
    path: PathBuf,
    pager_options: PagerOptions,
}

pub enum PagerOptions {
    Less(LessPagerOptions),
    Custom(CustomPagerOptions),
}

pub struct LessPagerOptions {
    pub follow: bool,
}

pub struct CustomPagerOptions {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Error, Diagnostic)]
pub enum PagerError {
    #[error(transparent)]
    CtrlCError(#[from] ctrlc::Error),

    #[error("Could not run pager")]
    #[diagnostic(code(less::command_spawn))]
    CommandSpawn(#[source] std::io::Error),

    #[error("Pager exited with non-zero status: {0}")]
    #[diagnostic(code(less::non_zero_exit))]
    NonZeroExit(std::process::ExitStatus),
}

impl Pager {
    pub const fn new(path: PathBuf, pager_options: PagerOptions) -> Self {
        Self { path, pager_options }
    }
}

impl Present for Pager {
    async fn present(&self) -> Result<()> {
        ctrlc::set_handler(|| {}).map_err(PagerError::CtrlCError)?;

        let mut command = match &self.pager_options {
            PagerOptions::Less(less) => get_less_pager_command(less.follow, &self.path),
            PagerOptions::Custom(custom) => {
                get_custom_pager_command(custom.command.clone(), custom.args.clone(), &self.path)
            }
        };

        let status = command.status().map_err(PagerError::CommandSpawn)?;

        status.success().then_some(()).ok_or(PagerError::NonZeroExit(status))?;

        Ok(())
    }
}

fn get_less_pager_command(follow: bool, path: &PathBuf) -> Command {
    let mut args = vec![
        "--ignore-case".to_string(),
        "--RAW-CONTROL-CHARS".to_string(),
        "--".to_string(), // End of option arguments
    ];

    if follow {
        args.insert(0, "+F".to_string());
    }

    let mut cmd = Command::new("less");

    cmd.env("LESSSECURE", "1").args(&args).arg(path);

    cmd
}

fn get_custom_pager_command(command: String, args: Vec<String>, path: &Path) -> Command {
    let replaced_args = replace_file_placeholder(args, path.to_str().unwrap());

    let mut cmd = Command::new(command);

    cmd.args(replaced_args);

    cmd
}

fn replace_file_placeholder(args: Vec<String>, path: &str) -> Vec<String> {
    args.into_iter().map(|arg| arg.replace("[FILE]", path)).collect()
}
