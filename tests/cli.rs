use assert_cmd::Command;
use std::fs;

fn cmd() -> Command {
    let mut c = Command::cargo_bin("claude-bar").unwrap();
    c.env("HOME", "/nonexistent");
    c.env_remove("CLAUDE_BAR_CONFIG");
    c.env_remove("XDG_CONFIG_HOME");
    c.env_remove("CLAUDE_BAR");
    c.env_remove("CLAUDE_BAR_ICON_SET");
    c
}

#[test]
fn demo_produces_output() {
    let output = cmd()
        .arg("--demo")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn demo_default_matches_baseline() {
    let baseline = include_str!("baselines/default.txt");
    cmd()
        .arg("--demo")
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_preset_minimal_matches_baseline() {
    let baseline = include_str!("baselines/minimal.txt");
    cmd()
        .args(["--demo", "--preset", "minimal"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_preset_compact_matches_baseline() {
    let baseline = include_str!("baselines/compact.txt");
    cmd()
        .args(["--demo", "--preset", "compact"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_preset_full_matches_baseline() {
    let baseline = include_str!("baselines/full.txt");
    cmd()
        .args(["--demo", "--preset", "full"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_no_icons_matches_baseline() {
    let baseline = include_str!("baselines/noicons.txt");
    cmd()
        .args(["--demo", "--no-icons"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_icon_set_fa_matches_baseline() {
    let baseline = include_str!("baselines/fa.txt");
    cmd()
        .args(["--demo", "--icon-set", "fa"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn demo_custom_elements_matches_baseline() {
    let baseline = include_str!("baselines/custom.txt");
    cmd()
        .args(["--demo", "--elements", "model,cost"])
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn list_prints_to_stderr() {
    let output = cmd()
        .arg("--list")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("PRESETS"));
    assert!(stderr.contains("ELEMENTS"));
}

#[test]
fn help_shows_usage() {
    let output = cmd()
        .arg("--help")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("claude-bar"));
}

#[test]
fn invalid_preset_exits_with_error() {
    let output = cmd()
        .args(["--demo", "--preset", "nonexistent"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown preset"));
}

#[test]
fn stdin_demo_json_matches_baseline() {
    let demo_json = fs::read_to_string("demo-status.json").unwrap();
    let baseline = include_str!("baselines/demo-json.txt");
    cmd()
        .write_stdin(demo_json)
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn stdin_empty_json_produces_no_output() {
    let baseline = include_str!("baselines/empty-json.txt");
    cmd()
        .write_stdin("{}")
        .assert()
        .success()
        .stdout(baseline);
}

#[test]
fn stdin_invalid_json_silent_failure() {
    let output = cmd()
        .write_stdin("not valid json")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
