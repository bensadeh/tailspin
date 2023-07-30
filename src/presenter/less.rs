use crate::presenter::Present;
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
    fn present(&self) {
        pass_ctrl_c_events_to_child_process();

        let args = get_args(self.follow);
        let status = Command::new("less")
            .env("LESSSECURE", "1")
            .args(args.as_slice())
            .arg(self.file_path.clone())
            .status();

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Failed to open file with less");
                }
            }
            Err(err) => {
                eprintln!("Failed to run less: {}", err);
            }
        }
    }
}

fn pass_ctrl_c_events_to_child_process() {
    // Without this handling, pressing Ctrl + C causes the program to exit
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
