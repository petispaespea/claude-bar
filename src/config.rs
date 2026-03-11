#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Model,
    Version,
    Gauge,
    Context,
    Cost,
    Lines,
    Duration,
    Cwd,
    ProjectDir,
    OutputStyle,
}

pub const ALL_ELEMENTS: &[Element] = &[
    Element::Model,
    Element::Version,
    Element::Gauge,
    Element::Context,
    Element::Cost,
    Element::Lines,
    Element::Duration,
    Element::Cwd,
    Element::ProjectDir,
    Element::OutputStyle,
];

use clap::Parser;

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
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Preset profile: minimal, compact, default, full"
    )]
    pub preset: Option<String>,

    #[arg(
        short,
        long,
        value_name = "LIST",
        help = "Comma-separated elements: model,version,gauge,ctx,cost,lines,duration,cwd,project,style"
    )]
    pub elements: Option<String>,

    #[arg(long, help = "List available elements and presets")]
    pub list: bool,

    #[arg(long, help = "Hide Nerd Font icons from elements")]
    pub no_icons: bool,

    #[arg(long, help = "Render with sample data (no stdin required)")]
    pub demo: bool,
}

fn preset_elements(name: &str) -> Option<Vec<Element>> {
    Some(match name {
        "minimal" => vec![
            Element::Model,
            Element::Gauge,
            Element::Context,
        ],
        "compact" => vec![
            Element::Model,
            Element::Gauge,
            Element::Context,
            Element::Cost,
            Element::Cwd,
        ],
        "default" => vec![
            Element::Model,
            Element::Gauge,
            Element::Context,
            Element::Duration,
            Element::Cwd,
            Element::ProjectDir,
            Element::OutputStyle,
        ],
        "full" => ALL_ELEMENTS.to_vec(),
        _ => return None,
    })
}

fn parse_custom_elements(spec: &str) -> Vec<Element> {
    spec.split(',')
        .filter_map(|s| match s.trim() {
            "model" => Some(Element::Model),
            "version" => Some(Element::Version),
            "gauge" => Some(Element::Gauge),
            "context" | "ctx" => Some(Element::Context),
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

pub fn resolve_elements(cli: &Cli) -> Vec<Element> {
    if let Some(ref spec) = cli.elements {
        return parse_custom_elements(spec);
    }

    if let Some(ref name) = cli.preset {
        if let Some(elems) = preset_elements(name) {
            return elems;
        }
        eprintln!("Unknown preset: {name}. Use --list to see available presets.");
        std::process::exit(1);
    }

    let env_val = std::env::var("CLAUDE_BAR").unwrap_or_else(|_| "default".into());
    if env_val.contains(',') {
        parse_custom_elements(&env_val)
    } else {
        preset_elements(&env_val).unwrap_or_else(|| ALL_ELEMENTS.to_vec())
    }
}

pub fn resolve_icons(cli: &Cli) -> bool {
    if cli.no_icons {
        return false;
    }
    std::env::var("CLAUDE_BAR_ICONS")
        .map(|v| v != "0" && v != "false")
        .unwrap_or(true)
}

pub fn print_list() {
    eprintln!("PRESETS");
    eprintln!("  minimal   model, gauge, context");
    eprintln!("  compact   model, gauge, context, cost, cwd");
    eprintln!("  default   model, gauge, context, duration, cwd, project, style");
    eprintln!("  full      all elements");
    eprintln!();
    eprintln!("ELEMENTS");
    eprintln!("  model          Model display name (e.g. Opus 4.6)");
    eprintln!("  version        Claude Code version");
    eprintln!("  gauge          Braille-dot context usage bar (color-coded)");
    eprintln!("  context, ctx   Context usage percentage");
    eprintln!("  cost           Session cost in USD");
    eprintln!("  lines          Lines added/removed this session");
    eprintln!("  duration, time API wait time");
    eprintln!("  cwd            Current working directory (shortened)");
    eprintln!("  project,       Project root directory (shortened)");
    eprintln!("    project_dir");
    eprintln!("  style,         Output style (hidden when \"default\")");
    eprintln!("    output_style");
}
