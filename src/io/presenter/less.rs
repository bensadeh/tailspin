use crate::io::presenter::Present;
use miette::Result;
use std::process::Command;
use std::{io, process};

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
    fn present(&self) -> Result<()> {
        pass_ctrl_c_events_to_child_process();

        let args = get_args(self.follow);
        let result = Command::new("less")
            .env("LESSSECURE", "1")
            .args(args.as_slice())
            .arg(self.file_path.clone())
            .status();

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    eprintln!("'less' command not found. Please ensure it is installed and on your PATH.");
                } else {
                    eprintln!("Failed to run less: {}", err);
                }
                process::exit(1);
            }
        }
    }
}

fn pass_ctrl_c_events_to_child_process() {
    // Without this handling, pressing Ctrl + C causes tailspin to exit
    // immediately instead of passing the signal down to the child process (less)
    ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
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
