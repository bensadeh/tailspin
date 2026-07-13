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
        .env_remove("TAILSPIN_EXTRAS")
        .env_remove("TAILSPIN_THEME");
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
fn file_input_highlights_with_all_extras() {
    let output = tspin()
        .args(["-p", "--extras", "ipv6,jvm-stack-trace", FIXTURE])
        .output()
        .unwrap();

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
fn stdin_edge_inputs_roundtrip() {
    let cases = [
        ("Hello null", "Hello \u{1b}[3;31mnull\u{1b}[0m"),
        ("Hello world", "Hello world"),
        ("", ""),
    ];

    for (input, expected) in cases {
        let output = tspin().write_stdin(input).output().unwrap();

        assert!(output.status.success());
        assert_eq!(stdout_of(&output).trim_end_matches('\n'), expected, "input: {input:?}");
    }
}

#[test]
fn disabling_the_keywords_group_turns_off_builtin_keywords() {
    let output = tspin()
        .args(["--disable", "keywords"])
        .write_stdin("Hello null\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(stdout_of(&output).trim_end_matches('\n'), "Hello null");
}

#[test]
fn highlight_flag_applies_even_with_keywords_disabled() {
    let output = tspin()
        .args(["--disable", "keywords", "--highlight", "red:foo"])
        .write_stdin("foo null\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        stdout_of(&output).trim_end_matches('\n'),
        "\u{1b}[31mfoo\u{1b}[0m null",
        "--highlight keywords must survive --disable keywords"
    );
}

#[test]
fn highlight_flag_accepts_bright_colors() {
    let output = tspin()
        .args(["--highlight", "bright_red:alert"])
        .write_stdin("alert raised\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        stdout_of(&output).contains("\u{1b}[91malert\u{1b}[0m"),
        "bright_red should paint with the bright ANSI code"
    );
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
fn missing_theme_path_errors_with_the_path() {
    let output = tspin()
        .args(["--theme", "/definitely/not/a/theme.toml"])
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
        .args(["--theme", theme.to_str().unwrap()])
        .write_stdin("x\n")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("unknown field `bogus`"));
}

#[cfg(unix)]
fn spawn_tspin(args: &[&str]) -> std::process::Child {
    std::process::Command::new(env!("CARGO_BIN_EXE_tspin"))
        .args(args)
        .env("XDG_CONFIG_HOME", EMPTY_CONFIG_DIR.path())
        .env_remove("TAILSPIN_PAGER")
        .env_remove("TAILSPIN_EXTRAS")
        .env_remove("TAILSPIN_THEME")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

#[cfg(unix)]
fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {
    use std::time::{Duration, Instant};

    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Whether any process has `pattern` in its command line.
#[cfg(unix)]
fn process_matching(pattern: &str) -> bool {
    std::process::Command::new("pgrep")
        .args(["-f", pattern])
        .output()
        .unwrap()
        .status
        .success()
}

/// The kill must be asserted first: a leaked child holds tspin's piped
/// stdout/stderr open, and reading them to EOF would block until it exits.
#[cfg(unix)]
fn output_once_killed(child: std::process::Child, what: &str, marker: &str) -> Output {
    wait_until(what, || !process_matching(marker));
    child.wait_with_output().unwrap()
}

#[cfg(unix)]
#[test]
fn quitting_the_pager_kills_the_exec_child() {
    let dir = tempfile::tempdir().unwrap();
    let exec_started = dir.path().join("exec-started");
    let quit_pager = dir.path().join("quit-pager");

    // `exec` keeps the uniquely named sleep as tspin's direct child (no grandchild).
    let nap = format!("31.{:04}", std::process::id() % 10_000);
    let exec = format!("touch {}; exec sleep {nap}", exec_started.display());
    // A pager that "quits" the moment the test creates the quit file. The
    // [FILE] token only satisfies routing validation; the script ignores it.
    let pager = format!(
        "sh -c 'until [ -e {} ]; do sleep 0.05; done' [FILE]",
        quit_pager.display()
    );

    let child = spawn_tspin(&["--exec", &exec, "--pager", &pager]);

    wait_until("the exec child to start", || exec_started.exists());
    std::fs::write(&quit_pager, "").unwrap();

    let marker = format!("sleep {nap}");
    let output = output_once_killed(child, "the exec child to be killed", &marker);
    assert!(output.status.success(), "quitting the pager is a clean exit");
}

#[cfg(unix)]
#[test]
fn failing_pager_spawn_kills_the_exec_child() {
    let nap = format!("32.{:04}", std::process::id() % 10_000);
    let exec = format!("exec sleep {nap}");

    let mut child = spawn_tspin(&["--exec", &exec, "--pager", "definitely-not-a-real-pager [FILE]"]);

    // Only tspin's exit proves the exec child was ever spawned
    child.wait().unwrap();

    let marker = format!("sleep {nap}");
    let output = output_once_killed(child, "the exec child to be killed", &marker);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("Could not run pager"));
}

#[cfg(unix)]
#[test]
fn stream_error_while_paging_kills_the_pager() {
    let dir = tempfile::tempdir().unwrap();
    let pager_started = dir.path().join("pager-started");

    // Unique sleep duration so pgrep identifies this test's pager and nothing else.
    let nap = format!("30.{:04}", std::process::id() % 10_000);
    let pager = format!("sh -c 'touch {}; exec sleep {nap}' [FILE]", pager_started.display());

    let child = spawn_tspin(&["--exec", "sleep 1; exit 3", "--pager", &pager]);

    wait_until("the pager to start", || pager_started.exists());

    let marker = format!("sleep {nap}");
    let output = output_once_killed(child, "the pager to be killed", &marker);
    assert_eq!(output.status.code(), Some(1));
    assert!(stderr_of(&output).contains("--exec command failed (exit status: 3)"));
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
        .env_remove("TAILSPIN_THEME")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    // A started pager proves the SIGINT handler is installed: Pager::spawn()
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
        .env_remove("TAILSPIN_THEME")
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
fn completions_print_for_every_supported_shell() {
    for shell in ["bash", "elvish", "fish", "powershell", "zsh"] {
        let output = tspin().args(["--completions", shell]).output().unwrap();

        assert!(output.status.success(), "--completions {shell} failed");
        assert!(
            stdout_of(&output).contains("tspin"),
            "--completions {shell} printed no completions"
        );
    }
}

#[test]
fn generated_default_theme_matches_the_committed_file() {
    let output = tspin().arg("--generate-default-theme").output().unwrap();

    assert!(output.status.success());
    let committed = std::fs::read_to_string("default-theme.toml").unwrap();
    assert_eq!(
        stdout_of(&output),
        committed,
        "default-theme.toml is stale — run util/generate_default_theme.sh"
    );
}

#[test]
fn generated_default_theme_is_a_noop() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    let generated = tspin().arg("--generate-default-theme").output().unwrap();
    std::fs::write(&theme, stdout_of(&generated)).unwrap();

    let with_theme = tspin()
        .args(["-p", FIXTURE, "--theme", theme.to_str().unwrap()])
        .output()
        .unwrap();
    let without_theme = tspin().args(["-p", FIXTURE]).output().unwrap();

    assert!(with_theme.status.success());
    assert_eq!(stdout_of(&with_theme), stdout_of(&without_theme));
}

#[test]
fn custom_theme_overrides_default_style() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    std::fs::write(&theme, "[numbers]\nstyle = { fg = \"green\" }\n").unwrap();

    let output = tspin()
        .args(["--theme", theme.to_str().unwrap()])
        .write_stdin("count 42\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        stdout_of(&output).contains("\x1b[32m42\x1b[0m"),
        "numbers should be green, not the default cyan"
    );
}

#[test]
fn tailspin_theme_env_var_loads_the_theme() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    std::fs::write(&theme, "[numbers]\nstyle = { fg = \"green\" }\n").unwrap();

    let output = tspin()
        .env("TAILSPIN_THEME", theme.to_str().unwrap())
        .write_stdin("count 42\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        stdout_of(&output).contains("\x1b[32m42\x1b[0m"),
        "the theme from TAILSPIN_THEME should apply"
    );
}

#[test]
fn appdata_is_the_theme_fallback_when_home_is_unset() {
    // The %APPDATA% branch is Windows-only in practice, but the lookup is
    // plain env vars, so it is exercised on every platform.
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("tailspin")).unwrap();
    std::fs::write(
        dir.path().join("tailspin/theme.toml"),
        "[numbers]\nstyle = { fg = \"green\" }\n",
    )
    .unwrap();

    let output = tspin()
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("HOME")
        .env("APPDATA", dir.path())
        .write_stdin("count 42\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(
        stdout_of(&output).contains("\x1b[32m42\x1b[0m"),
        "the theme under %APPDATA% should apply"
    );
}

#[test]
fn ipv4_and_ipv6_theme_tables_style_independently() {
    let dir = tempfile::tempdir().unwrap();
    let theme = dir.path().join("theme.toml");
    std::fs::write(
        &theme,
        "[ipv4]\nseparator = { fg = \"magenta\" }\n\n[ipv6]\nseparator = { fg = \"green\" }\n",
    )
    .unwrap();

    let output = tspin()
        .args(["--extras", "ipv6", "--theme", theme.to_str().unwrap()])
        .write_stdin("ipv4 1.2.3.4 ipv6 2001:db8::1\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = stdout_of(&output);
    assert!(
        stdout.contains("\x1b[35m.\x1b[0m"),
        "ipv4 separators should take the [ipv4] color"
    );
    assert!(
        stdout.contains("\x1b[32m::\x1b[0m"),
        "ipv6 separators should take the [ipv6] color"
    );
}
