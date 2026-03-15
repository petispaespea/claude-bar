use clap::{CommandFactory, Parser};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Element {
    Model,
    Version,
    Context,
    Tokens,
    Cache,
    Cost,
    Lines,
    Duration,
    Cwd,
    ProjectDir,
    OutputStyle,
    Alert,
    DailyCost,
    BurnRate,
    SpendRate,
    SessionCount,
    DailyBudget,
    TokPerDollar,
    CacheHitRate,
    CostVsAvg,
    CtxTrend,
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
pub const CONTEXT_ICONS: Icons     = Icons { none: "",     oct: "\u{f4ed} ", fa: "\u{eeb2} " };
pub const TOKENS_ICONS: Icons      = Icons { none: "",     oct: "\u{f4df} ", fa: "\u{f292} " };
pub const CACHE_ICONS: Icons       = Icons { none: "",     oct: "\u{f49b} ", fa: "\u{f1c0} " };
pub const COST_ICONS: Icons        = Icons { none: "",     oct: "\u{f439} ", fa: "\u{f09d} " };
pub const LINES_ICONS: Icons       = Icons { none: "",     oct: "\u{f4d2} ", fa: "\u{f05f} " };
pub const DURATION_ICONS: Icons    = Icons { none: "api:", oct: "\u{f4e3} ", fa: "\u{f254} " };
pub const CWD_ICONS: Icons         = Icons { none: "cwd:", oct: "\u{f413} ", fa: "\u{f114} " };
pub const PROJECT_ICONS: Icons     = Icons { none: "proj:", oct: "\u{f46d} ", fa: "\u{f015} " };
pub const STYLE_ICONS: Icons       = Icons { none: "",     oct: "\u{f48f} ", fa: "\u{f1fc} " };
pub const ALERT_ICONS: Icons       = Icons { none: "",     oct: "\u{f421} ", fa: "\u{f071} " };
pub const SESSION_CT_ICONS: Icons  = Icons { none: "#",    oct: "\u{f4a5} ", fa: "\u{f0c5} " };
pub const COST_VS_AVG_ICONS: Icons = Icons { none: "",     oct: "\u{f4a8} ", fa: "\u{f080} " };

const ALL_ELEMENTS: &[Element] = &[
    Element::Model,
    Element::Version,
    Element::Context,
    Element::Tokens,
    Element::Cache,
    Element::Cost,
    Element::Lines,
    Element::Duration,
    Element::Cwd,
    Element::ProjectDir,
    Element::OutputStyle,
    Element::Alert,
    Element::DailyCost,
    Element::BurnRate,
    Element::SpendRate,
    Element::SessionCount,
    Element::DailyBudget,
    Element::TokPerDollar,
    Element::CacheHitRate,
    Element::CostVsAvg,
    Element::CtxTrend,
];

pub const CORE_ELEMENT_NAMES: &[&str] = &[
    "model", "version", "context", "tokens", "cache",
    "cost", "lines", "duration", "cwd", "project", "style", "alert",
];

pub const STATS_ELEMENT_NAMES: &[&str] = &[
    "daily_cost", "burn_rate", "spend_rate", "session_count",
    "daily_budget", "tok_per_dollar", "cache_hit_rate", "cost_vs_avg", "ctx_trend",
];

pub const ALL_ELEMENT_NAMES: &[&str] = &[
    "model", "version", "context", "tokens", "cache",
    "cost", "lines", "duration", "cwd", "project", "style", "alert",
    "daily_cost", "burn_rate", "spend_rate", "session_count",
    "daily_budget", "tok_per_dollar", "cache_hit_rate", "cost_vs_avg", "ctx_trend",
];

const _: () = assert!(ALL_ELEMENTS.len() == ALL_ELEMENT_NAMES.len());
const _: () = assert!(CORE_ELEMENT_NAMES.len() + STATS_ELEMENT_NAMES.len() == ALL_ELEMENT_NAMES.len());

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
        help = "Comma-separated elements; use --- for line break (model, version, context/ctx, tokens, cache, cost, lines, duration/time, cwd, project/project_dir, style/output_style, alert, daily_cost, burn_rate, spend_rate, session_count, daily_budget, tok_per_dollar, cache_hit_rate/cache_hit, cost_vs_avg, ctx_trend)"
    )]
    pub elements: Option<String>,

    #[arg(long, help = "Show available elements, presets, and bar styles")]
    pub info: bool,

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

    #[arg(long, help = "Show usage statistics")]
    pub stats: bool,

    #[arg(long, value_name = "N", default_value = "7", help = "Stats lookback in days")]
    pub stats_days: u64,

    #[arg(long, value_name = "PATH", help = "Filter stats to a specific project")]
    pub stats_project: Option<String>,

    #[arg(long, help = "Delete the stats log file")]
    pub stats_clear: bool,

    #[arg(long, help = "Confirm destructive operations (required for --stats-clear)")]
    pub yes: bool,
}

pub fn build_cli() -> clap::Command {
    Cli::command()
}

pub(crate) fn preset_elements(name: &str) -> Option<Vec<Element>> {
    Some(match name {
        "minimal" => vec![Element::Model, Element::Context, Element::Alert],
        "compact" => vec![
            Element::Model,
            Element::Context,
            Element::Cost,
            Element::Cwd,
            Element::Alert,
        ],
        "default" => vec![
            Element::Model,
            Element::Context,
            Element::Tokens,
            Element::Duration,
            Element::Cwd,
            Element::ProjectDir,
            Element::OutputStyle,
            Element::Alert,
        ],
        "full" => ALL_ELEMENTS.to_vec(),
        _ => return None,
    })
}

fn parse_element(s: &str) -> Option<Element> {
    match s.trim() {
        "model" => Some(Element::Model),
        "version" => Some(Element::Version),
        "context" | "ctx" => Some(Element::Context),
        "tokens" => Some(Element::Tokens),
        "cache" => Some(Element::Cache),
        "cost" => Some(Element::Cost),
        "lines" => Some(Element::Lines),
        "duration" | "time" => Some(Element::Duration),
        "cwd" => Some(Element::Cwd),
        "project" | "project_dir" => Some(Element::ProjectDir),
        "style" | "output_style" => Some(Element::OutputStyle),
        "alert" => Some(Element::Alert),
        "daily_cost" => Some(Element::DailyCost),
        "burn_rate" => Some(Element::BurnRate),
        "spend_rate" => Some(Element::SpendRate),
        "session_count" => Some(Element::SessionCount),
        "daily_budget" => Some(Element::DailyBudget),
        "tok_per_dollar" => Some(Element::TokPerDollar),
        "cache_hit_rate" | "cache_hit" => Some(Element::CacheHitRate),
        "cost_vs_avg" => Some(Element::CostVsAvg),
        "ctx_trend" => Some(Element::CtxTrend),
        _ => None,
    }
}

pub const LINE_BREAK: &str = "---";

fn split_into_lines<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<Vec<Element>> {
    let mut lines = vec![Vec::new()];
    for token in tokens {
        if token.trim() == LINE_BREAK {
            lines.push(Vec::new());
        } else if let Some(elem) = parse_element(token) {
            lines.last_mut().unwrap().push(elem);
        }
    }
    lines.retain(|l| !l.is_empty());
    lines
}

pub(crate) fn parse_elements(spec: &str) -> Vec<Vec<Element>> {
    split_into_lines(spec.split(','))
}

pub(crate) fn env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

pub(crate) fn debug(msg: &str) {
    if std::env::var("CLAUDE_BAR_DEBUG").is_ok() {
        eprintln!("[claude-bar] {msg}");
    }
}

pub fn resolve_elements(cli: &Cli, toml_layout: Option<&[String]>) -> Vec<Vec<Element>> {
    if let Some(ref spec) = cli.elements {
        debug(&format!("elements: using --elements {spec}"));
        return parse_elements(spec);
    }
    if let Some(ref name) = cli.preset {
        debug(&format!("elements: using --preset {name}"));
        return vec![preset_elements(name).unwrap_or_else(|| {
            eprintln!("Unknown preset: {name}. Use --info to see available presets.");
            std::process::exit(1);
        })];
    }
    if let Some(val) = env("CLAUDE_BAR") {
        if val.contains(',') {
            debug(&format!("elements: using $CLAUDE_BAR={val}"));
            return parse_elements(&val);
        }
        if let Some(elems) = preset_elements(&val) {
            debug(&format!("elements: using $CLAUDE_BAR preset {val}"));
            return vec![elems];
        }
    }
    if let Some(layout) = toml_layout.filter(|l| !l.is_empty()) {
        debug(&format!("elements: using TOML layout [{}]", layout.join(", ")));
        return split_into_lines(layout.iter().map(|s| s.as_str()));
    }
    debug("elements: using built-in default preset");
    vec![preset_elements("default").unwrap()]
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

pub fn print_info() {
    eprint!(
        "\
PRESETS
  minimal        model, context, alert
  compact        model, context, cost, cwd, alert
  default        model, context, tokens, duration, cwd, project, style, alert
  full           all elements

ELEMENTS
  model          Model display name (e.g. Opus 4.6)
  version        Claude Code version
  context, ctx   Context usage bar + percentage (configurable)
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
  alert          Conditional badges (ctx exceeded, ctx high, budget)

STATS ELEMENTS (require [stats] enabled = true)
  daily_cost     Sum of session costs today
  burn_rate      Cost per hour (API duration)
  spend_rate     Cost per hour (wall clock)
  session_count  Number of sessions today
  daily_budget   Daily spend limit with progress bar
  tok_per_dollar Output tokens per dollar
  cost_vs_avg    Current session cost vs historical average
  ctx_trend      Context usage direction over last 10 renders

INPUT-ONLY ELEMENTS
  cache_hit_rate Cache hit percentage (no stats required)
    cache_hit

BAR STYLES (for context element)
  braille        Sub-cell braille dots (default, highest resolution)
  block          Filled/empty blocks (▰▱)
  shade          Gradient shading (█▓▒░)
  ascii          Plain ASCII ([###-------])

ICON SETS
  octicons       Octicons (default)
  fontawesome    Font Awesome (alias: fa)
  none, off      No icons (text prefixes for paths)

MULTI-LINE LAYOUT
  Use \"---\" in layout.elements to split into multiple lines.
  Example: elements = [\"model\", \"context\", \"---\", \"cost\", \"duration\"]
  With --elements: --elements model,context,---,cost,duration
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cli() -> Cli {
        Cli {
            preset: None,
            elements: None,
            info: false,
            no_icons: false,
            icon_set: None,
            demo: false,
            setup: false,
            completions: None,
            print_default_config: false,
            config: None,
            stats: false,
            stats_days: 7,
            stats_project: None,
            stats_clear: false,
            yes: false,
        }
    }

    #[test]
    fn test_parse_elements_all_element_names() {
        let result = parse_elements(
            "model,version,context,tokens,cache,cost,lines,duration,cwd,project,style,alert",
        );
        assert_eq!(result.len(), 1);
        let line = &result[0];
        assert_eq!(line.len(), 12);
        assert!(line.contains(&Element::Model));
        assert!(line.contains(&Element::Version));
        assert!(line.contains(&Element::Context));
        assert!(line.contains(&Element::Tokens));
        assert!(line.contains(&Element::Cache));
        assert!(line.contains(&Element::Cost));
        assert!(line.contains(&Element::Lines));
        assert!(line.contains(&Element::Duration));
        assert!(line.contains(&Element::Cwd));
        assert!(line.contains(&Element::ProjectDir));
        assert!(line.contains(&Element::OutputStyle));
        assert!(line.contains(&Element::Alert));
    }

    #[test]
    fn test_parse_elements_aliases() {
        let result = parse_elements("ctx,time,project_dir,output_style");
        let line = &result[0];
        assert_eq!(line.len(), 4);
        assert_eq!(line[0], Element::Context);
        assert_eq!(line[1], Element::Duration);
        assert_eq!(line[2], Element::ProjectDir);
        assert_eq!(line[3], Element::OutputStyle);
    }

    #[test]
    fn test_parse_elements_mixed_aliases_and_names() {
        let result = parse_elements("model,ctx,time,cwd,alert");
        let line = &result[0];
        assert_eq!(line.len(), 5);
        assert_eq!(line[0], Element::Model);
        assert_eq!(line[1], Element::Context);
        assert_eq!(line[2], Element::Duration);
        assert_eq!(line[3], Element::Cwd);
        assert_eq!(line[4], Element::Alert);
    }

    #[test]
    fn test_parse_elements_unknown_names_dropped() {
        let result = parse_elements("model,unknown1,context,unknown2,cost");
        let line = &result[0];
        assert_eq!(line.len(), 3);
        assert_eq!(line[0], Element::Model);
        assert_eq!(line[1], Element::Context);
        assert_eq!(line[2], Element::Cost);
    }

    #[test]
    fn test_parse_elements_empty_string() {
        let result = parse_elements("");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_elements_whitespace_handling() {
        let result = parse_elements(" model , context , alert ");
        let line = &result[0];
        assert_eq!(line.len(), 3);
        assert!(line.contains(&Element::Model));
        assert!(line.contains(&Element::Context));
        assert!(line.contains(&Element::Alert));
    }

    #[test]
    fn test_preset_elements_minimal() {
        let result = preset_elements("minimal");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0], Element::Model);
        assert_eq!(elements[1], Element::Context);
        assert_eq!(elements[2], Element::Alert);
    }

    #[test]
    fn test_preset_elements_compact() {
        let result = preset_elements("compact");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 5);
        assert!(elements.contains(&Element::Model));
        assert!(elements.contains(&Element::Context));
        assert!(elements.contains(&Element::Cost));
        assert!(elements.contains(&Element::Cwd));
        assert!(elements.contains(&Element::Alert));
    }

    #[test]
    fn test_preset_elements_default() {
        let result = preset_elements("default");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 8);
        assert!(elements.contains(&Element::Model));
        assert!(elements.contains(&Element::Context));
        assert!(elements.contains(&Element::Tokens));
        assert!(elements.contains(&Element::Duration));
        assert!(elements.contains(&Element::Cwd));
        assert!(elements.contains(&Element::ProjectDir));
        assert!(elements.contains(&Element::OutputStyle));
        assert!(elements.contains(&Element::Alert));
    }

    #[test]
    fn test_preset_elements_full() {
        let result = preset_elements("full");
        assert!(result.is_some());
        let elements = result.unwrap();
        assert_eq!(elements.len(), 21);
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
    fn test_parse_elements_new_stats_elements() {
        let result = parse_elements(
            "daily_cost,burn_rate,spend_rate,session_count,daily_budget,tok_per_dollar,cache_hit_rate,cost_vs_avg,ctx_trend",
        );
        let line = &result[0];
        assert_eq!(line.len(), 9);
        assert!(line.contains(&Element::DailyCost));
        assert!(line.contains(&Element::SpendRate));
        assert!(line.contains(&Element::CacheHitRate));
    }

    #[test]
    fn test_parse_elements_cache_hit_alias() {
        let result = parse_elements("cache_hit");
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], Element::CacheHitRate);
    }

    #[test]
    fn test_preset_elements_full_includes_stats() {
        let result = preset_elements("full").unwrap();
        assert!(result.contains(&Element::DailyCost));
        assert!(result.contains(&Element::BurnRate));
        assert!(result.contains(&Element::SpendRate));
        assert!(result.contains(&Element::CacheHitRate));
        assert!(result.contains(&Element::CtxTrend));
    }

    #[test]
    fn test_parse_elements_line_separator() {
        let result = parse_elements("model,context,---,cost,duration");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![Element::Model, Element::Context]);
        assert_eq!(result[1], vec![Element::Cost, Element::Duration]);
    }

    #[test]
    fn test_resolve_elements_cli_precedence() {
        let mut cli = test_cli();
        cli.preset = Some("minimal".into());
        cli.elements = Some("model,cost".into());
        let result = resolve_elements(&cli, None);
        let line = &result[0];
        assert_eq!(line.len(), 2);
        assert_eq!(line[0], Element::Model);
        assert_eq!(line[1], Element::Cost);
    }

    #[test]
    fn test_resolve_elements_preset_precedence() {
        let mut cli = test_cli();
        cli.preset = Some("compact".into());
        let result = resolve_elements(&cli, None);
        let line = &result[0];
        assert_eq!(line.len(), 5);
        assert!(line.contains(&Element::Cost));
        assert!(line.contains(&Element::Alert));
    }

    #[test]
    fn test_resolve_icon_mode_no_icons_flag() {
        let mut cli = test_cli();
        cli.no_icons = true;
        assert_eq!(resolve_icon_mode(&cli), IconMode::None);
    }

    #[test]
    fn test_resolve_icon_mode_fontawesome() {
        let mut cli = test_cli();
        cli.icon_set = Some("fa".into());
        assert_eq!(resolve_icon_mode(&cli), IconMode::FontAwesome);
    }

    #[test]
    fn test_resolve_icon_mode_octicons_default() {
        let cli = test_cli();
        assert_eq!(resolve_icon_mode(&cli), IconMode::Octicons);
    }

    #[test]
    fn test_all_element_names_parse_to_matching_elements() {
        for (elem, name) in ALL_ELEMENTS.iter().zip(ALL_ELEMENT_NAMES.iter()) {
            assert_eq!(
                parse_element(name),
                Some(*elem),
                "ALL_ELEMENT_NAMES entry '{name}' does not parse to matching ALL_ELEMENTS entry"
            );
        }
    }
}
