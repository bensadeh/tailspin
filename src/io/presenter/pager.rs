use crate::io::presenter::Present;
use anyhow::Result;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::process::Command;

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

#[derive(Debug, Error)]
pub enum PagerError {
    #[error("Could not set up signal handler")]
    SignalSetup(#[source] std::io::Error),

    #[error("Could not run pager")]
    CommandSpawn(#[source] std::io::Error),

    #[error("Pager exited with non-zero status: {0}")]
    NonZeroExit(std::process::ExitStatus),
}

impl Pager {
    pub const fn new(path: PathBuf, pager_options: PagerOptions) -> Self {
        Self { path, pager_options }
    }
}

impl Present for Pager {
    async fn present(&self) -> Result<()> {
        #[cfg(unix)]
        let _sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .map_err(PagerError::SignalSetup)?;

        let mut command = match &self.pager_options {
            PagerOptions::Less(less) => get_less_pager_command(less.follow, &self.path),
            PagerOptions::Custom(custom) => {
                get_custom_pager_command(custom.command.clone(), custom.args.clone(), &self.path)
            }
        };

        let status = command.status().await.map_err(PagerError::CommandSpawn)?;

        status.success().then_some(()).ok_or(PagerError::NonZeroExit(status))?;

        Ok(())
    }
}

fn get_less_pager_command(follow: bool, path: &Path) -> Command {
    let mut args = vec![
        "--ignore-case".to_string(),
        "--RAW-CONTROL-CHARS".to_string(),
        "--".to_string(), // End of option arguments
    ];

    if follow {
        args.insert(0, "+F".to_string());
    }

    let mut cmd = Command::new("less");

    cmd.env("LESSSECURE", "1").args(&args).arg(path).kill_on_drop(true);

    cmd
}

fn get_custom_pager_command(command: String, args: Vec<String>, path: &Path) -> Command {
    let replaced_args = replace_file_placeholder(args, &path.to_string_lossy());

    let mut cmd = Command::new(command);

    cmd.args(replaced_args).kill_on_drop(true);

    cmd
}

fn replace_file_placeholder(args: Vec<String>, path: &str) -> Vec<String> {
    args.into_iter().map(|arg| arg.replace("[FILE]", path)).collect()
}
