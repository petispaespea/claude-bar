use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

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
fn config_layout_hides_element() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[layout]").unwrap();
    writeln!(config_file, "elements = [\"context\"]").unwrap();
    config_file.flush().unwrap();

    let output = cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(!stdout.contains("Opus"));
    assert!(stdout.contains("30%"));
}

#[test]
fn config_custom_symbol() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "symbol = \"🤖 \"").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("🤖"));
}

#[test]
fn config_custom_style_red_bold() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "style = \"red bold\"").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("[31m"))
        .stdout(predicate::str::contains("[1m"));
}

#[test]
fn config_custom_layout() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[layout]").unwrap();
    writeln!(
        config_file,
        "elements = [\"context\", \"model\"]"
    )
    .unwrap();
    config_file.flush().unwrap();

    let output = cmd()
        .env("CLAUDE_BAR", "notapreset")
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());

    let context_pos = stdout.find("30%").unwrap_or(usize::MAX);
    let model_pos = stdout.find("Opus").unwrap_or(0);
    assert!(
        context_pos < model_pos,
        "context should appear before model"
    );
}

#[test]
fn config_custom_separator() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "separator = \" | \"").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(" | "));
}

#[test]
fn config_invalid_toml() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model").unwrap();
    writeln!(config_file, "disabled = true").unwrap();
    config_file.flush().unwrap();

    let output = cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn config_missing_file() {
    cmd()
        .arg("--demo")
        .arg("--config")
        .arg("/tmp/nonexistent-abc123-xyz.toml")
        .assert()
        .success();
}

#[test]
fn config_unknown_fields() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "symbol = \"TEST \"").unwrap();
    writeln!(config_file, "unknown_field = \"value\"").unwrap();
    writeln!(config_file, "new_feature = true").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST"));
}

#[test]
fn config_layout_hides_multiple_elements() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[layout]").unwrap();
    writeln!(config_file, "elements = [\"cost\", \"duration\"]").unwrap();
    config_file.flush().unwrap();

    let output = cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(!stdout.contains("Opus"));
    assert!(!stdout.contains("⣿"));
    assert!(stdout.contains("$4.11"));
}

#[test]
fn config_print_default_roundtrip() {
    let default_config = cmd()
        .arg("--print-default-config")
        .output()
        .unwrap();

    let mut config_file = NamedTempFile::new().unwrap();
    config_file.write_all(&default_config.stdout).unwrap();
    config_file.flush().unwrap();

    let first_run = cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let second_run = cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    assert!(first_run.status.success());
    assert!(!first_run.stdout.is_empty());
    assert_eq!(first_run.stdout, second_run.stdout);
}

#[test]
fn config_style_cyan_dim() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[context]").unwrap();
    writeln!(config_file, "style = \"cyan dim\"").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("[36m"))
        .stdout(predicate::str::contains("[2m"));
}

#[test]
fn config_multiple_custom_fields() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "symbol = \"M:\"").unwrap();
    writeln!(config_file, "style = \"red\"").unwrap();
    writeln!(config_file, "[context]").unwrap();
    writeln!(config_file, "symbol = \"C:\"").unwrap();
    writeln!(config_file, "style = \"cyan\"").unwrap();
    config_file.flush().unwrap();

    cmd()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("M:"))
        .stdout(predicate::str::contains("C:"))
        .stdout(predicate::str::contains("[31m"))
        .stdout(predicate::str::contains("[36m"));
}
