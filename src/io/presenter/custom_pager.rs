use crate::io::presenter::Present;
use miette::{IntoDiagnostic, Result, WrapErr, miette};
use shell_words::split;
use std::path::PathBuf;
use std::process::Command;

pub struct CustomPager {
    path: PathBuf,
    command: String,
}

impl CustomPager {
    pub const fn new(path: PathBuf, command: String) -> Self {
        Self { path, command }
    }
}

impl Present for CustomPager {
    async fn present(&self) -> Result<()> {
        ctrlc::set_handler(|| {})
            .into_diagnostic()
            .wrap_err("Failed to set Ctrl-C handler")?;

        let string_path = self
            .path
            .as_path()
            .to_str()
            .ok_or_else(|| miette!("Custom pager command is empty"))?;

        let raw_command = self.command.replace("[FILE]", string_path);
        let mut parts = split(&raw_command)
            .into_diagnostic()
            .wrap_err("Failed to parse custom pager command")?;

        if parts.is_empty() {
            return Err(miette!("Custom pager command is empty"));
        }

        let binary = parts.remove(0);

        let status = Command::new(binary)
            .args(parts)
            .status()
            .into_diagnostic()
            .wrap_err("Failed to use custom pager")?;

        status
            .success()
            .then_some(())
            .ok_or_else(|| miette!("The custom pager exited with a non-zero status: {}", status))
    }
}
