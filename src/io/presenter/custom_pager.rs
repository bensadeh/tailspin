use crate::io::presenter::Present;
use miette::{miette, IntoDiagnostic, WrapErr};
use shell_words::split;
use std::process::Command;

pub struct CustomPager {
    temp_file: String,
    command: String,
}

impl CustomPager {
    pub fn get_presenter(temp_file: String, command: String) -> Box<dyn Present + Send> {
        Box::new(Self { temp_file, command })
    }
}

impl Present for CustomPager {
    fn present(&self) -> miette::Result<()> {
        ctrlc::set_handler(|| {})
            .into_diagnostic()
            .wrap_err("Failed to set Ctrl-C handler")?;

        let raw_command = self.command.replace("[FILE]", &self.temp_file);
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
