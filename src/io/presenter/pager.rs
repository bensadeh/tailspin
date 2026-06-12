use crate::config::{CustomPagerOptions, LessOptions};
use anyhow::Result;
use std::path::Path;
use tempfile::TempPath;
use thiserror::Error;
use tokio::process::Command;

pub struct Pager {
    path: TempPath,
    options: PagerOptions,
}

pub enum PagerOptions {
    Less(LessOptions),
    Custom(CustomPagerOptions),
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
    pub const fn new(path: TempPath, options: PagerOptions) -> Self {
        Self { path, options }
    }

    pub async fn present(&self) -> Result<()> {
        // Installs a process-global SIGINT handler so Ctrl+C (sent to the whole
        // process group) stops at the pager instead of also killing tspin behind it.
        #[cfg(unix)]
        let _ignore_sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .map_err(PagerError::SignalSetup)?;

        let mut command = match &self.options {
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
