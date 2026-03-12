use clap::{CommandFactory, Parser};

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Model,
    Version,
    Gauge,
    Context,
    Tokens,
    Cache,
    Cost,
    Lines,
    Duration,
    Cwd,
    ProjectDir,
    OutputStyle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum IconMode {
    None,
    Octicons,
    FontAwesome,
}

const ALL_ELEMENTS: &[Element] = &[
    Element::Model,
    Element::Version,
    Element::Gauge,
    Element::Context,
    Element::Tokens,
    Element::Cache,
    Element::Cost,
    Element::Lines,
    Element::Duration,
    Element::Cwd,
    Element::ProjectDir,
    Element::OutputStyle,
];

#[derive(Parser)]
#[command(
    name = "claude-bar",
    about = "Configurable status line for Claude Code",
    long_about = "Renders a configurable status line for Claude Code.\n\n\
        Reads JSON from stdin (provided by Claude Code) and outputs a \
        formatted single-line status with model info, context usage, \
        cost, and more.\n\n\
        SETUP\n  \
        Add to ~/.claude/settings.json:\n    \
        \"statusLine\": {\n      \
        \"type\": \"command\",\n      \
        \"command\": \"/path/to/claude-bar\"\n    \
        }\n\n\
        Configuration priority: CLI flags > CLAUDE_BAR env var > default preset"
)]
pub struct Cli {
    #[arg(short, long, value_name = "NAME", help = "Preset: minimal, compact, default, full")]
    pub preset: Option<String>,

    #[arg(short, long, value_name = "LIST", help = "Comma-separated elements")]
    pub elements: Option<String>,

    #[arg(long, help = "List available elements and presets")]
    pub list: bool,

    #[arg(long, help = "Hide Nerd Font icons")]
    pub no_icons: bool,

    #[arg(long = "icon-set", value_name = "SET", help = "Icon set: octicons (default), fontawesome")]
    pub icon_set: Option<String>,

    #[arg(long, help = "Render with sample data (no stdin required)")]
    pub demo: bool,

    #[arg(long, help = "Add statusLine to ~/.claude/settings.json")]
    pub setup: bool,

    #[arg(long, value_name = "SHELL", help = "Generate shell completions (bash, zsh, fish, elvish, powershell)")]
    pub completions: Option<String>,
}

pub fn build_cli() -> clap::Command {
    Cli::command()
}

fn preset_elements(name: &str) -> Option<Vec<Element>> {
    Some(match name {
        "minimal" => vec![Element::Model, Element::Gauge, Element::Context],
        "compact" => vec![
            Element::Model, Element::Gauge, Element::Context, Element::Cost, Element::Cwd,
        ],
        "default" => vec![
            Element::Model, Element::Gauge, Element::Context, Element::Tokens,
            Element::Duration, Element::Cwd, Element::ProjectDir, Element::OutputStyle,
        ],
        "full" => ALL_ELEMENTS.to_vec(),
        _ => return None,
    })
}

fn parse_elements(spec: &str) -> Vec<Element> {
    spec.split(',')
        .filter_map(|s| match s.trim() {
            "model" => Some(Element::Model),
            "version" => Some(Element::Version),
            "gauge" => Some(Element::Gauge),
            "context" | "ctx" => Some(Element::Context),
            "tokens" => Some(Element::Tokens),
            "cache" => Some(Element::Cache),
            "cost" => Some(Element::Cost),
            "lines" => Some(Element::Lines),
            "duration" | "time" => Some(Element::Duration),
            "cwd" => Some(Element::Cwd),
            "project" | "project_dir" => Some(Element::ProjectDir),
            "style" | "output_style" => Some(Element::OutputStyle),
            _ => None,
        })
        .collect()
}

fn env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

pub fn resolve_elements(cli: &Cli) -> Vec<Element> {
    if let Some(ref spec) = cli.elements {
        return parse_elements(spec);
    }
    if let Some(ref name) = cli.preset {
        return preset_elements(name).unwrap_or_else(|| {
            eprintln!("Unknown preset: {name}. Use --list to see available presets.");
            std::process::exit(1);
        });
    }
    let val = env("CLAUDE_BAR").unwrap_or_else(|| "default".into());
    if val.contains(',') {
        parse_elements(&val)
    } else {
        preset_elements(&val).unwrap_or_else(|| ALL_ELEMENTS.to_vec())
    }
}

pub fn resolve_icon_mode(cli: &Cli) -> IconMode {
    if cli.no_icons || env("CLAUDE_BAR_ICONS").is_some_and(|v| v == "0" || v == "false") {
        return IconMode::None;
    }
    let env_set = env("CLAUDE_BAR_ICON_SET");
    let set = cli.icon_set.as_deref().or(env_set.as_deref());
    match set {
        Some("fontawesome" | "fa") => IconMode::FontAwesome,
        _ => IconMode::Octicons,
    }
}

pub fn print_list() {
    eprint!("\
PRESETS
  minimal        model, gauge, context
  compact        model, gauge, context, cost, cwd
  default        model, gauge, context, tokens, duration, cwd, project, style
  full           all elements

ELEMENTS
  model          Model display name (e.g. Opus 4.6)
  version        Claude Code version
  gauge          Braille-dot context usage bar (color-coded)
  context, ctx   Context usage percentage
  tokens         Input/output token counts
  cache          Cache read/write token counts
  cost           Session cost in USD
  lines          Lines added/removed this session
  duration, time API wait time
  cwd            Current working directory (shortened)
  project,       Project root directory (shortened)
    project_dir
  style,         Output style (hidden when \"default\")
    output_style

ICON SETS
  octicons       Octicons (default)
  fontawesome    Font Awesome (alias: fa)
");
}
