use std::process::Command;

pub(crate) fn open_file(path: &str, follow: bool) {
    pass_ctrl_c_events_gracefully_to_child_process();

    let args = get_args(follow);
    let status = Command::new("less")
        .args(args.as_slice())
        .arg(path)
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

fn pass_ctrl_c_events_gracefully_to_child_process() {
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
