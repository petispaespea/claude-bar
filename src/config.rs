use clap::{CommandFactory, Parser};

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconMode {
    None,
    Octicons,
    FontAwesome,
}

pub struct Icons {
    pub none: &'static str,
    pub oct: &'static str,
    pub fa: &'static str,
}

pub const MODEL_ICONS: Icons       = Icons { none: "",     oct: "\u{f4be} ", fa: "\u{ee0d} " };
pub const VERSION_ICONS: Icons     = Icons { none: "",     oct: "\u{f412} ", fa: "\u{f02b} " };
pub const GAUGE_ICONS: Icons       = Icons { none: "",     oct: "\u{f4ed} ", fa: "\u{ef0d} " };
pub const CONTEXT_ICONS: Icons     = Icons { none: "",     oct: "\u{f463} ", fa: "\u{eeb2} " };
pub const TOKENS_ICONS: Icons      = Icons { none: "",     oct: "\u{f4df} ", fa: "\u{f292} " };
pub const CACHE_ICONS: Icons       = Icons { none: "",     oct: "\u{f49b} ", fa: "\u{f1c0} " };
pub const COST_ICONS: Icons        = Icons { none: "",     oct: "\u{f439} ", fa: "\u{f09d} " };
pub const LINES_ICONS: Icons       = Icons { none: "",     oct: "\u{f4d2} ", fa: "\u{f05f} " };
pub const DURATION_ICONS: Icons    = Icons { none: "api:", oct: "\u{f4e3} ", fa: "\u{f254} " };
pub const CWD_ICONS: Icons         = Icons { none: "cwd:", oct: "\u{f413} ", fa: "\u{f114} " };
pub const PROJECT_ICONS: Icons     = Icons { none: "proj:", oct: "\u{f46d} ", fa: "\u{f015} " };
pub const STYLE_ICONS: Icons       = Icons { none: "",     oct: "\u{f48f} ", fa: "\u{f1fc} " };

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
    long_about = "Configurable status line for Claude Code.\n\n\
        Run --setup to configure, or --demo to preview.\n\n\
        Configuration priority (highest to lowest):\n  \
        1. CLI flags (--elements, --preset, --icon-set)\n  \
        2. Environment variables (CLAUDE_BAR, CLAUDE_BAR_ICON_SET)\n  \
        3. TOML config file\n  \
        4. Built-in defaults\n\n\
        Config file resolution:\n  \
        1. --config <path>\n  \
        2. $CLAUDE_BAR_CONFIG\n  \
        3. $XDG_CONFIG_HOME/claude-bar.toml\n  \
        4. ~/.config/claude-bar.toml"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Preset: minimal, compact, default, full"
    )]
    pub preset: Option<String>,

    #[arg(
        short,
        long,
        value_name = "LIST",
        help = "Comma-separated elements (model, version, gauge, context/ctx, tokens, cache, cost, lines, duration/time, cwd, project/project_dir, style/output_style)"
    )]
    pub elements: Option<String>,

    #[arg(long, help = "List available elements and presets")]
    pub list: bool,

    #[arg(long, help = "Hide Nerd Font icons")]
    pub no_icons: bool,

    #[arg(
        long = "icon-set",
        value_name = "SET",
        help = "Icon set: octicons (default), fontawesome/fa, none/off"
    )]
    pub icon_set: Option<String>,

    #[arg(long, help = "Render with sample data (no stdin required)")]
    pub demo: bool,

    #[arg(long, help = "Add statusLine to ~/.claude/settings.json")]
    pub setup: bool,

    #[arg(
        long,
        value_name = "SHELL",
        help = "Generate shell completions (bash, zsh, fish, elvish, powershell)"
    )]
    pub completions: Option<String>,

    #[arg(long, help = "Path to TOML config file")]
    pub config: Option<std::path::PathBuf>,

    #[arg(long, help = "Print default TOML config to stdout")]
    pub print_default_config: bool,
}

pub fn build_cli() -> clap::Command {
    Cli::command()
}

pub(crate) fn preset_elements(name: &str) -> Option<Vec<Element>> {
    Some(match name {
        "minimal" => vec![Element::Model, Element::Gauge, Element::Context],
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
            Element::Tokens,
            Element::Duration,
            Element::Cwd,
            Element::ProjectDir,
            Element::OutputStyle,
        ],
        "full" => ALL_ELEMENTS.to_vec(),
        _ => return None,
    })
}

fn parse_element(s: &str) -> Option<Element> {
    match s.trim() {
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
    }
}

pub(crate) fn parse_elements(spec: &str) -> Vec<Element> {
    spec.split(',').filter_map(parse_element).collect()
}

pub(crate) fn env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

pub(crate) fn debug(msg: &str) {
    if std::env::var("CLAUDE_BAR_DEBUG").is_ok() {
        eprintln!("[claude-bar] {msg}");
    }
}

pub fn resolve_elements(cli: &Cli, toml_layout: Option<&[String]>) -> Vec<Element> {
    if let Some(ref spec) = cli.elements {
        debug(&format!("elements: using --elements {spec}"));
        return parse_elements(spec);
    }
    if let Some(ref name) = cli.preset {
        debug(&format!("elements: using --preset {name}"));
        return preset_elements(name).unwrap_or_else(|| {
            eprintln!("Unknown preset: {name}. Use --list to see available presets.");
            std::process::exit(1);
        });
    }
    if let Some(val) = env("CLAUDE_BAR") {
        if val.contains(',') {
            debug(&format!("elements: using $CLAUDE_BAR={val}"));
            return parse_elements(&val);
        }
        if let Some(elems) = preset_elements(&val) {
            debug(&format!("elements: using $CLAUDE_BAR preset {val}"));
            return elems;
        }
    }
    if let Some(layout) = toml_layout.filter(|l| !l.is_empty()) {
        debug(&format!("elements: using TOML layout [{}]", layout.join(", ")));
        return layout.iter().filter_map(|s| parse_element(s)).collect();
    }
    debug("elements: using built-in default preset");
    preset_elements("default").unwrap()
}

pub fn resolve_icon_mode(cli: &Cli) -> IconMode {
    if cli.no_icons {
        debug("icons: disabled via --no-icons");
        return IconMode::None;
    }
    let env_set = env("CLAUDE_BAR_ICON_SET");
    let set = cli.icon_set.as_deref().or(env_set.as_deref());
    let mode = match set {
        Some("none" | "off") => IconMode::None,
        Some("fontawesome" | "fa") => IconMode::FontAwesome,
        _ => IconMode::Octicons,
    };
    if let Some(src) = set {
        debug(&format!("icons: {src} → {mode:?}"));
    } else {
        debug(&format!("icons: default → {mode:?}"));
    }
    mode
}

pub fn print_list() {
    eprint!(
        "\
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
  none, off      No icons (text prefixes for paths)
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elements_all_element_names() {
        let result = parse_elements(
            "model,version,gauge,context,tokens,cache,cost,lines,duration,cwd,project,style",
        );
        assert_eq!(result.len(), 12);
        assert!(result.contains(&Element::Model));
        assert!(result.contains(&Element::Version));
        assert!(result.contains(&Element::Gauge));
        assert!(result.contains(&Element::Context));
        assert!(result.contains(&Element::Tokens));
        assert!(result.contains(&Element::Cache));
        assert!(result.contains(&Element::Cost));
        assert!(result.contains(&Element::Lines));
        assert!(result.contains(&Element::Duration));
        assert!(result.contains(&Element::Cwd));
        assert!(result.contains(&Element::ProjectDir));
        assert!(result.contains(&Element::OutputStyle));
    }

    #[test]
    fn test_parse_elements_aliases() {
        let result = parse_elements("ctx,time,project_dir,output_style");
        assert_eq!(result.len(), 4);
        assert!(result.contains(&Element::Context));
        assert!(result.contains(&Element::Duration));
        assert!(result.contains(&Element::ProjectDir));
        assert!(result.contains(&Element::OutputStyle));
    }

    #[test]
    fn test_parse_elements_mixed_aliases_and_names() {
        let result = parse_elements("model,ctx,gauge,time,cwd");
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], Element::Model);
        assert_eq!(result[1], Element::Context);
        assert_eq!(result[2], Element::Gauge);
        assert_eq!(result[3], Element::Duration);
        assert_eq!(result[4], Element::Cwd);
    }

    #[test]
    fn test_parse_elements_unknown_names_dropped() {
        let result = parse_elements("model,unknown1,gauge,unknown2,cost");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Element::Model);
        assert_eq!(result[1], Element::Gauge);
        assert_eq!(result[2], Element::Cost);
    }

    #[test]
    fn test_parse_elements_empty_string() {
        let result = parse_elements("");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_elements_whitespace_handling() {
        let result = parse_elements(" model , gauge , context ");
        assert_eq!(result.len(), 3);
        assert!(result.contains(&Element::Model));
        assert!(result.contains(&Element::Gauge));
        assert!(result.contains(&Element::Context));
    }

    #[test]
    fn test_preset_elements_minimal() {
        let result = preset_elements("minimal");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0], Element::Model);
        assert_eq!(elements[1], Element::Gauge);
        assert_eq!(elements[2], Element::Context);
    }

    #[test]
    fn test_preset_elements_compact() {
        let result = preset_elements("compact");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 5);
        assert!(elements.contains(&Element::Model));
        assert!(elements.contains(&Element::Gauge));
        assert!(elements.contains(&Element::Context));
        assert!(elements.contains(&Element::Cost));
        assert!(elements.contains(&Element::Cwd));
    }

    #[test]
    fn test_preset_elements_default() {
        let result = preset_elements("default");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 8);
        assert!(elements.contains(&Element::Model));
        assert!(elements.contains(&Element::Gauge));
        assert!(elements.contains(&Element::Context));
        assert!(elements.contains(&Element::Tokens));
        assert!(elements.contains(&Element::Duration));
        assert!(elements.contains(&Element::Cwd));
        assert!(elements.contains(&Element::ProjectDir));
        assert!(elements.contains(&Element::OutputStyle));
    }

    #[test]
    fn test_preset_elements_full() {
        let result = preset_elements("full");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 12);
        for elem in ALL_ELEMENTS.iter() {
            assert!(elements.contains(elem));
        }
    }

    #[test]
    fn test_preset_elements_unknown() {
        let result = preset_elements("unknown_preset");
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_elements_cli_precedence() {
        let cli = Cli {
            preset: Some("minimal".into()),
            elements: Some("model,cost".into()),
            list: false,
            no_icons: false,
            icon_set: None,
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
        };
        let result = resolve_elements(&cli, None);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Element::Model);
        assert_eq!(result[1], Element::Cost);
    }

    #[test]
    fn test_resolve_elements_preset_precedence() {
        let cli = Cli {
            preset: Some("compact".into()),
            elements: None,
            list: false,
            no_icons: false,
            icon_set: None,
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
        };
        let result = resolve_elements(&cli, None);
        assert_eq!(result.len(), 5);
        assert!(result.contains(&Element::Cost));
    }

    #[test]
    fn test_resolve_icon_mode_no_icons_flag() {
        let cli = Cli {
            preset: None,
            elements: None,
            list: false,
            no_icons: true,
            icon_set: None,
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
        };
        assert_eq!(resolve_icon_mode(&cli), IconMode::None);
    }

    #[test]
    fn test_resolve_icon_mode_fontawesome() {
        let cli = Cli {
            preset: None,
            elements: None,
            list: false,
            no_icons: false,
            icon_set: Some("fa".into()),
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
        };
        assert_eq!(resolve_icon_mode(&cli), IconMode::FontAwesome);
    }

    #[test]
    fn test_resolve_icon_mode_octicons_default() {
        let cli = Cli {
            preset: None,
            elements: None,
            list: false,
            no_icons: false,
            icon_set: None,
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
        };
        assert_eq!(resolve_icon_mode(&cli), IconMode::Octicons);
    }
}
