use clap::Parser;
use serde::Deserialize;
use std::io::Read;

#[derive(Parser)]
#[command(
    name = "claude-statusline",
    about = "Configurable status line for Claude Code",
    long_about = "Renders a configurable status line for Claude Code.\n\n\
        Reads JSON from stdin (provided by Claude Code) and outputs a \
        formatted single-line status with model info, context usage, \
        cost, and more.\n\n\
        SETUP\n  \
        Add to ~/.claude/settings.json:\n    \
        \"statusLine\": {\n      \
        \"type\": \"command\",\n      \
        \"command\": \"/path/to/claude-statusline\"\n    \
        }\n\n\
        Configuration priority: CLI flags > CLAUDE_STATUSLINE env var > default preset"
)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Preset profile: minimal, compact, default, full"
    )]
    preset: Option<String>,

    #[arg(
        short,
        long,
        value_name = "LIST",
        help = "Comma-separated elements: model,version,gauge,ctx,cost,lines,duration,cwd,project,style"
    )]
    elements: Option<String>,

    #[arg(long, help = "List available elements and presets")]
    list: bool,
}

#[derive(Deserialize)]
struct Input {
    model: Option<Model>,
    context_window: Option<ContextWindow>,
    cost: Option<Cost>,
    cwd: Option<String>,
    version: Option<String>,
    exceeds_200k_tokens: Option<bool>,
    output_style: Option<OutputStyle>,
    workspace: Option<Workspace>,
}

#[derive(Deserialize)]
struct Model {
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct ContextWindow {
    used_percentage: Option<f64>,
}

#[derive(Deserialize)]
struct Cost {
    total_cost_usd: Option<f64>,
    total_lines_added: Option<i64>,
    total_lines_removed: Option<i64>,
    total_duration_ms: Option<u64>,
    total_api_duration_ms: Option<u64>,
}

#[derive(Deserialize)]
struct OutputStyle {
    name: Option<String>,
}

#[derive(Deserialize)]
struct Workspace {
    project_dir: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Element {
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

const ALL_ELEMENTS: &[Element] = &[
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
            Element::Version,
            Element::Gauge,
            Element::Context,
            Element::Cost,
            Element::Lines,
            Element::Duration,
            Element::Cwd,
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

fn resolve_elements(cli: &Cli) -> Vec<Element> {
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

    let env_val = std::env::var("CLAUDE_STATUSLINE").unwrap_or_else(|_| "default".into());
    if env_val.contains(',') {
        parse_custom_elements(&env_val)
    } else {
        preset_elements(&env_val).unwrap_or_else(|| ALL_ELEMENTS.to_vec())
    }
}

fn print_list() {
    eprintln!("PRESETS");
    eprintln!("  minimal   model, gauge, context");
    eprintln!("  compact   model, gauge, context, cost, cwd");
    eprintln!("  default   model, version, gauge, context, cost, lines, duration, cwd");
    eprintln!("  full      all elements");
    eprintln!();
    eprintln!("ELEMENTS");
    eprintln!("  model          Model display name (e.g. Opus 4.6)");
    eprintln!("  version        Claude Code version");
    eprintln!("  gauge          Braille-dot context usage bar (color-coded)");
    eprintln!("  context, ctx   Context usage percentage");
    eprintln!("  cost           Session cost in USD");
    eprintln!("  lines          Lines added/removed this session");
    eprintln!("  duration, time Session uptime and API wait time");
    eprintln!("  cwd            Current working directory (shortened)");
    eprintln!("  project,       Project root directory (shortened)");
    eprintln!("    project_dir");
    eprintln!("  style,         Output style (hidden when \"default\")");
    eprintln!("    output_style");
}

const BRAILLE_LEVELS: [char; 9] = [
    '\u{2800}', // ⠀ empty
    '\u{2840}', // ⡀
    '\u{2844}', // ⡄
    '\u{2846}', // ⡆
    '\u{2847}', // ⡇
    '\u{28C7}', // ⣇
    '\u{28E7}', // ⣧
    '\u{28F7}', // ⣷
    '\u{28FF}', // ⣿ full
];

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const BG_RED: &str = "\x1b[41m";
const WHITE: &str = "\x1b[97m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn braille_gauge(percentage: f64, width: usize, fill_color: &str) -> String {
    let fill = percentage / 100.0 * width as f64;
    let mut gauge = String::new();

    for i in 0..width {
        let level = (fill - i as f64).clamp(0.0, 1.0);
        let idx = (level * 8.0).round() as usize;
        if idx > 0 {
            gauge.push_str(fill_color);
        } else {
            gauge.push_str(DIM);
        }
        gauge.push(BRAILLE_LEVELS[idx]);
        gauge.push_str(RESET);
    }

    gauge
}

fn pct_color(pct: f64) -> &'static str {
    if pct >= 80.0 {
        RED
    } else if pct >= 50.0 {
        YELLOW
    } else {
        GREEN
    }
}

fn format_duration(ms: u64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    if hours > 0 {
        format!("{hours}h{mins:02}m")
    } else {
        format!("{mins}m")
    }
}

fn shorten_path(path: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let shortened = if !home.is_empty() && path.starts_with(&home) {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
    };

    let parts: Vec<&str> = shortened.split('/').collect();
    if parts.len() <= 3 {
        return shortened;
    }
    let last_two = &parts[parts.len() - 2..];
    format!("…/{}", last_two.join("/"))
}

fn render_element(elem: Element, input: &Input) -> Option<String> {
    match elem {
        Element::Model => {
            let name = input.model.as_ref()?.display_name.as_ref()?;
            Some(format!("{CYAN}{name}{RESET}"))
        }
        Element::Version => {
            let v = input.version.as_ref()?;
            Some(format!("{DIM}v{v}{RESET}"))
        }
        Element::Gauge => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            let color = pct_color(pct);
            let gauge = braille_gauge(pct, 10, color);
            let mut result = gauge;
            if input.exceeds_200k_tokens.unwrap_or(false) {
                result.push_str(&format!(" {BG_RED}{WHITE} CTX EXCEEDED {RESET}"));
            }
            Some(result)
        }
        Element::Context => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            let color = pct_color(pct);
            Some(format!("{color}{pct:.0}%{RESET}"))
        }
        Element::Cost => {
            let c = input.cost.as_ref()?.total_cost_usd?;
            Some(format!("{DIM}${c:.2}{RESET}"))
        }
        Element::Lines => {
            let cost = input.cost.as_ref()?;
            let added = cost.total_lines_added.unwrap_or(0);
            let removed = cost.total_lines_removed.unwrap_or(0);
            if added == 0 && removed == 0 {
                return None;
            }
            Some(format!("{GREEN}+{added}{RESET}/{RED}-{removed}{RESET}"))
        }
        Element::Duration => {
            let cost = input.cost.as_ref()?;
            let session = cost.total_duration_ms.map(format_duration)?;
            let result = match cost.total_api_duration_ms.map(format_duration) {
                Some(api) => format!("{DIM}{session} (api {api}){RESET}"),
                None => format!("{DIM}{session}{RESET}"),
            };
            Some(result)
        }
        Element::Cwd => {
            let p = input.cwd.as_ref()?;
            Some(format!("{DIM}{}{RESET}", shorten_path(p)))
        }
        Element::ProjectDir => {
            let p = input.workspace.as_ref()?.project_dir.as_ref()?;
            Some(format!("{DIM}proj:{}{RESET}", shorten_path(p)))
        }
        Element::OutputStyle => {
            let name = input.output_style.as_ref()?.name.as_ref()?;
            if name == "default" {
                return None;
            }
            Some(format!("{DIM}[{name}]{RESET}"))
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        print_list();
        return;
    }

    let elements = resolve_elements(&cli);

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
        .filter_map(|e| render_element(*e, &input))
        .collect();

    print!("{}", parts.join("  "));
}
