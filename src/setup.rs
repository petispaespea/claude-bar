use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;

fn settings_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".claude").join("settings.json")
}

pub fn run() {
    let bin = std::env::current_exe().unwrap_or_else(|e| {
        eprintln!("Could not determine binary path: {e}");
        std::process::exit(1);
    });
    let bin = bin.to_string_lossy();
    let path = settings_path();

    let mut settings: Map<String, Value> = if path.exists() {
        let data = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Could not read {}: {e}", path.display());
            std::process::exit(1);
        });
        serde_json::from_str(&data).unwrap_or_else(|e| {
            eprintln!("Could not parse {}: {e}", path.display());
            std::process::exit(1);
        })
    } else {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        Map::new()
    };

    let status_line = serde_json::json!({
        "type": "command",
        "command": bin.as_ref()
    });

    let had_existing = settings.contains_key("statusLine");
    settings.insert("statusLine".into(), status_line);

    let json = serde_json::to_string_pretty(&Value::Object(settings)).unwrap();
    fs::write(&path, json + "\n").unwrap_or_else(|e| {
        eprintln!("Could not write {}: {e}", path.display());
        std::process::exit(1);
    });

    if had_existing {
        eprintln!("Updated statusLine in {}", path.display());
    } else {
        eprintln!("Added statusLine to {}", path.display());
    }
    eprintln!("  command: {bin}");
    eprintln!("Restart Claude Code to apply.");
}
