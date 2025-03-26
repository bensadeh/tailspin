use crate::io::presenter::Present;
use ctrlc::Error;
use miette::{miette, IntoDiagnostic, WrapErr};
use std::process::Command;

pub struct Less {
    file_path: String,
    follow: bool,
}

impl Less {
    pub fn get_presenter(file_path: String, follow: bool) -> Box<dyn Present + Send> {
        Box::new(Self { file_path, follow })
    }
}

impl Present for Less {
    fn present(&self) -> miette::Result<()> {
        pass_ctrl_c_events_to_child_process()
            .into_diagnostic()
            .wrap_err("Failed to set Ctrl-C handler")?;

        let args = get_args(self.follow);
        let status = Command::new("less")
            .env("LESSSECURE", "1")
            .args(&args)
            .arg(&self.file_path)
            .status()
            .into_diagnostic()
            .wrap_err("Failed to execute 'less' command")?;

        status
            .success()
            .then_some(())
            .ok_or_else(|| miette!("The 'less' command exited with a non-zero status: {}", status))
    }
}

fn pass_ctrl_c_events_to_child_process() -> Result<(), Error> {
    // Without this, pressing Ctrl + C causes tailspin to exit immediately
    // instead of passing the signal down to the child process (less)
    ctrlc::set_handler(|| {})
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
