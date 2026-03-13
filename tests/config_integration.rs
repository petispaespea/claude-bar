use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn config_disabled_module() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "disabled = true").unwrap();
    config_file.flush().unwrap();

    let output = Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(!stdout.contains("󰧑"));
    assert!(!stdout.contains("Opus"));
}

#[test]
fn config_custom_symbol() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "symbol = \"🤖 \"").unwrap();
    config_file.flush().unwrap();

    Command::cargo_bin("claude-bar")
        .unwrap()
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

    Command::cargo_bin("claude-bar")
        .unwrap()
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
        "elements = [\"context\", \"model\", \"gauge\"]"
    )
    .unwrap();
    config_file.flush().unwrap();

    let output = Command::cargo_bin("claude-bar")
        .unwrap()
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

    Command::cargo_bin("claude-bar")
        .unwrap()
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

    let output = Command::cargo_bin("claude-bar")
        .unwrap()
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
    Command::cargo_bin("claude-bar")
        .unwrap()
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

    Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("TEST"));
}

#[test]
fn config_multiple_disabled_modules() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[model]").unwrap();
    writeln!(config_file, "disabled = true").unwrap();
    writeln!(config_file, "[version]").unwrap();
    writeln!(config_file, "disabled = true").unwrap();
    writeln!(config_file, "[gauge]").unwrap();
    writeln!(config_file, "disabled = true").unwrap();
    config_file.flush().unwrap();

    let output = Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(!stdout.contains("󰧑"));
    assert!(!stdout.contains("Opus"));
    assert!(!stdout.contains("1.40.6"));
    assert!(!stdout.contains("⣿"));
}

#[test]
fn config_print_default_roundtrip() {
    let default_output = Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--demo")
        .output()
        .unwrap();

    let default_config = Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--print-default-config")
        .output()
        .unwrap();

    let mut config_file = NamedTempFile::new().unwrap();
    config_file.write_all(&default_config.stdout).unwrap();
    config_file.flush().unwrap();

    let roundtrip_output = Command::cargo_bin("claude-bar")
        .unwrap()
        .arg("--demo")
        .arg("--config")
        .arg(config_file.path())
        .output()
        .unwrap();

    assert_eq!(default_output.stdout, roundtrip_output.stdout);
}

#[test]
fn config_style_cyan_dim() {
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, "[context]").unwrap();
    writeln!(config_file, "style = \"cyan dim\"").unwrap();
    config_file.flush().unwrap();

    Command::cargo_bin("claude-bar")
        .unwrap()
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

    Command::cargo_bin("claude-bar")
        .unwrap()
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
