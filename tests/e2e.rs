//! End-to-end tests that spawn the real `tspin` binary.

use assert_cmd::Command;
use std::process::Output;
use std::sync::LazyLock;
use tempfile::TempDir;

const FIXTURE: &str = "tests/files/e2e.log";

/// Empty config dir shared by all tests so a theme.toml on the developer's
/// machine cannot leak into the asserted output.
static EMPTY_CONFIG_DIR: LazyLock<TempDir> = LazyLock::new(|| tempfile::tempdir().unwrap());

fn tspin() -> Command {
    let mut cmd = Command::cargo_bin("tspin").unwrap();
    cmd.env("XDG_CONFIG_HOME", EMPTY_CONFIG_DIR.path())
        .env_remove("TAILSPIN_PAGER")
        .env_remove("TAILSPIN_EXTRAS");
    cmd
}

fn stdout_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Makes ANSI escapes visible in snapshots.
fn readable(output: &Output) -> String {
    stdout_of(output).replace('\x1b', "␛")
}

#[test]
fn file_input_highlights_with_default_theme() {
    let output = tspin().args(["-p", FIXTURE]).output().unwrap();

    assert!(output.status.success());
    insta::assert_snapshot!(readable(&output));
}

#[test]
fn stdin_input_is_highlighted() {
    let output = tspin().write_stdin("status 200 null\n").output().unwrap();

    assert!(output.status.success());
    insta::assert_snapshot!(readable(&output));
}

#[test]
fn custom_pager_receives_highlighted_file() {
    let output = tspin().arg(FIXTURE).args(["--pager", "cat [FILE]"]).output().unwrap();

    assert!(output.status.success());
    let stdout = stdout_of(&output);
    assert!(stdout.contains("\x1b["), "pager output should be highlighted");
    assert!(stdout.contains("Starting server"));
}

#[test]
fn exec_output_is_highlighted() {
    let output = tspin().args(["-p", "--exec", "echo code 200"]).output().unwrap();

    assert!(output.status.success());
    assert!(stdout_of(&output).contains("\x1b[36m200\x1b[0m"));
}

#[test]
fn exec_failure_fails_tspin_but_keeps_output() {
    let output = tspin()
        .args(["-p", "--exec", "echo partial result; exit 3"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stdout_of(&output).contains("partial result"));
    assert!(stderr_of(&output).contains("--exec command failed (exit status: 3)"));
}

#[test]
fn explicit_exec_beats_piped_stdin() {
    let output = tspin()
        .args(["-p", "--exec", "echo from-exec"])
        .write_stdin("from-stdin\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = stdout_of(&output);
    assert!(stdout.contains("from-exec"));
    assert!(!stdout.contains("from-stdin"));
}

#[test]
fn file_and_exec_together_error() {
    let output = tspin().arg(FIXTURE).args(["--exec", "echo hi"]).output().unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("Cannot read from both file and"));
}

#[test]
fn missing_file_errors() {
    let output = tspin().arg("definitely/not/a/file.log").output().unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("No such file or directory"));
}

#[test]
fn missing_config_path_errors_with_the_path() {
    let output = tspin()
        .args(["--config-path", "/definitely/not/a/theme.toml"])
        .write_stdin("x\n")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("could not read /definitely/not/a/theme.toml"));
}

#[test]
fn malformed_theme_errors_with_unknown_field() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    std::fs::write(&theme, "bogus = 1\n").unwrap();

    let output = tspin()
        .args(["--config-path", theme.to_str().unwrap()])
        .write_stdin("x\n")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("unknown field `bogus`"));
}

#[cfg(unix)]
#[test]
fn tspin_survives_sigint_while_pager_runs() {
    use std::time::{Duration, Instant};

    let dir = tempfile::tempdir().unwrap();
    let marker = dir.path().join("pager-started");
    let pager = format!("sh -c 'touch {}; sleep 2; cat [FILE]'", marker.display());

    let child = std::process::Command::new(env!("CARGO_BIN_EXE_tspin"))
        .arg(FIXTURE)
        .args(["--pager", &pager])
        .env("XDG_CONFIG_HOME", EMPTY_CONFIG_DIR.path())
        .env_remove("TAILSPIN_PAGER")
        .env_remove("TAILSPIN_EXTRAS")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // A started pager proves the SIGINT handler is installed: present()
    // registers it before spawning the pager.
    let deadline = Instant::now() + Duration::from_secs(10);
    while !marker.exists() {
        assert!(Instant::now() < deadline, "pager never started");
        std::thread::sleep(Duration::from_millis(20));
    }

    let kill = std::process::Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(kill.success());

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "tspin must survive SIGINT while paging");
    assert!(stdout_of(&output).contains("Starting server"));
}

#[cfg(unix)]
#[test]
fn pager_killed_by_ctrl_c_is_a_quiet_quit() {
    use std::time::{Duration, Instant};

    // `tail -f` never exits on its own and does not trap SIGINT.
    let child = std::process::Command::new(env!("CARGO_BIN_EXE_tspin"))
        .arg(FIXTURE)
        .args(["--pager", "tail -f [FILE]"])
        .env("XDG_CONFIG_HOME", EMPTY_CONFIG_DIR.path())
        .env_remove("TAILSPIN_PAGER")
        .env_remove("TAILSPIN_EXTRAS")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let pager_pid = {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let pgrep = std::process::Command::new("pgrep")
                .args(["-P", &child.id().to_string()])
                .output()
                .unwrap();
            let pid = String::from_utf8_lossy(&pgrep.stdout).trim().to_string();
            if !pid.is_empty() {
                break pid;
            }
            assert!(Instant::now() < deadline, "pager never started");
            std::thread::sleep(Duration::from_millis(20));
        }
    };

    let kill = std::process::Command::new("kill")
        .args(["-INT", &pager_pid])
        .status()
        .unwrap();
    assert!(kill.success());

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "Ctrl+C in the pager is a quit, not an error");
    assert_eq!(stderr_of(&output), "");
}

#[test]
fn custom_theme_overrides_default_style() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    std::fs::write(&theme, "[numbers]\nnumber = { fg = \"green\" }\n").unwrap();

    let output = tspin()
        .args(["--config-path", theme.to_str().unwrap()])
        .write_stdin("count 42\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        stdout_of(&output).contains("\x1b[32m42\x1b[0m"),
        "numbers should be green, not the default cyan"
    );
}
