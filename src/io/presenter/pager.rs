use crate::io::routing::{CustomPagerOptions, LessOptions};
use anyhow::Result;
use shared_child::SharedChild;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::sync::Arc;
use tempfile::TempPath;
use thiserror::Error;

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

    #[error("Could not wait for pager")]
    Wait(#[source] std::io::Error),

    #[error("Pager exited with non-zero status: {0}")]
    NonZeroExit(ExitStatus),
}

impl Pager {
    pub const fn new(path: TempPath, options: PagerOptions) -> Self {
        Self { path, options }
    }

    /// The temp file moves into the returned `PagerChild`, so it lives
    /// exactly as long as the pager can still read it.
    pub fn spawn(self) -> Result<PagerChild> {
        platform::ignore_sigint().map_err(PagerError::SignalSetup)?;

        let mut command = match &self.options {
            PagerOptions::Less(less) => get_less_pager_command(less.follow, &self.path),
            PagerOptions::Custom(custom) => {
                get_custom_pager_command(custom.command.clone(), custom.args.clone(), &self.path)
            }
        };

        let child = SharedChild::spawn(&mut command).map_err(PagerError::CommandSpawn)?;

        Ok(PagerChild {
            child: Arc::new(child),
            _path: self.path,
        })
    }
}

pub struct PagerChild {
    child: Arc<SharedChild>,
    _path: TempPath,
}

impl PagerChild {
    pub fn waiter(&self) -> PagerWaiter {
        PagerWaiter(self.child.clone())
    }

    pub fn kill(&self) {
        let _ = self.child.kill();
    }
}

/// Waits for the pager from another thread while `PagerChild` retains the
/// ability to kill it.
pub struct PagerWaiter(Arc<SharedChild>);

impl PagerWaiter {
    pub fn wait(self) -> Result<()> {
        let status = self.0.wait().map_err(PagerError::Wait)?;

        if status.success() || platform::interrupted_by_user(status) {
            Ok(())
        } else {
            Err(PagerError::NonZeroExit(status).into())
        }
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

    cmd.env("LESSSECURE", "1").args(&args).arg(path);

    cmd
}

fn get_custom_pager_command(command: String, args: Vec<String>, path: &Path) -> Command {
    let replaced_args = replace_file_placeholder(args, &path.to_string_lossy());

    let mut cmd = Command::new(command);

    cmd.args(replaced_args);

    cmd
}

fn replace_file_placeholder(args: Vec<String>, path: &str) -> Vec<String> {
    args.into_iter().map(|arg| arg.replace("[FILE]", path)).collect()
}

#[cfg(unix)]
mod platform {
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    // Registering this handler makes Ctrl+C (sent to the whole process group) stop at
    // the pager instead of killing tspin. The flag is never read; the handler stays
    // installed for the rest of the process.
    pub fn ignore_sigint() -> std::io::Result<()> {
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::new(AtomicBool::new(false))).map(|_| ())
    }

    pub fn interrupted_by_user(status: ExitStatus) -> bool {
        const SIGINT: i32 = 2;
        status.signal() == Some(SIGINT)
    }
}

#[cfg(windows)]
mod platform {
    use std::process::ExitStatus;

    pub fn ignore_sigint() -> std::io::Result<()> {
        Ok(())
    }

    pub fn interrupted_by_user(_status: ExitStatus) -> bool {
        false
    }
}
