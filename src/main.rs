mod config;
mod format;
mod render;

use clap::Parser;
use config::{Cli, resolve_elements, resolve_icons};
use render::render_element;
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
pub struct Input {
    pub model: Option<Model>,
    pub context_window: Option<ContextWindow>,
    pub cost: Option<Cost>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    pub exceeds_200k_tokens: Option<bool>,
    pub output_style: Option<OutputStyle>,
    pub workspace: Option<Workspace>,
}

#[derive(Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    pub used_percentage: Option<f64>,
}

#[derive(Deserialize)]
pub struct Cost {
    pub total_cost_usd: Option<f64>,
    pub total_lines_added: Option<i64>,
    pub total_lines_removed: Option<i64>,
    pub total_api_duration_ms: Option<u64>,
}

#[derive(Deserialize)]
pub struct OutputStyle {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub project_dir: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        config::print_list();
        return;
    }

    let elements = resolve_elements(&cli);
    let show_icons = resolve_icons(&cli);

    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        return;
    }

    let input: Input = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(_) => return,
    };

    let parts: Vec<String> = elements
        .iter()
        .filter_map(|e| render_element(*e, &input, show_icons))
        .collect();

    print!("{}", parts.join("  "));
}
