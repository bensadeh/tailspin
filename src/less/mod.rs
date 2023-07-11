use std::process::Command;

pub(crate) fn open_file_with_less(path: &str, follow: bool) {
    pass_ctrl_c_events_gracefully_to_child_process();

    let output = if follow {
        Command::new("less").arg("+F").arg(path).status()
    } else {
        Command::new("less").arg(path).status()
    };

    match output {
        Ok(status) => {
            if !status.success() {
                eprintln!("Failed to open file with less");
            }
        }
        Err(err) => {
            eprintln!("Failed to execute pager command: {}", err);
        }
    }
}

fn pass_ctrl_c_events_gracefully_to_child_process() {
    // Without this handling, pressing Ctrl + C causes the program to exit
    // immediately instead of passing the signal down to the child process (less)
    ctrlc::set_handler(|| {}).expect("Error setting Ctrl-C handler");
}
