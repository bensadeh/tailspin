use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[must_use]
pub fn build_binary() -> PathBuf {
    Command::new("cargo")
        .arg("build")
        .status()
        .expect("Failed to execute cargo build");

    PathBuf::from("./target/debug/tspin")
}

#[must_use]
pub fn run_binary_with_input(binary_path: PathBuf, input: &str) -> String {
    let mut child = Command::new(binary_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");

    String::from_utf8_lossy(&output.stdout).into_owned()
}
