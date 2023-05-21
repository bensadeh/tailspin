use std::{
    io::Write,
    process::{Command, Stdio},
};

fn main() {
    let text = "This is the text to be displayed by less.";

    run_less_with_input(text);

    println!("it works!")
}

fn run_less_with_input(input: &str) {
    let mut less_child = Command::new("less")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute 'less' command");

    if let Some(less_stdin) = less_child.stdin.as_mut() {
        less_stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to 'less' stdin");
    }

    less_child
        .wait()
        .expect("Failed to wait for 'less' command");
}
