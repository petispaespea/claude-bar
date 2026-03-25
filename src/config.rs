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
    WallTime,
    GitBranch,
    Cwd,
    ProjectDir,
    OutputStyle,
    Alert,
    ProjectTodayCost,
    BurnRate,
    SpendRate,
    DailyBudget,
    SessionTokPerDollar,
    CacheHitRate,
    CostVsAvg,
    CtxTrend,
    AvgDailyCost,
    SessionId,
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
pub const WALL_TIME_ICONS: Icons   = Icons { none: "wall:", oct: "\u{f4e3} ", fa: "\u{f254} " };
pub const GIT_BRANCH_ICONS: Icons  = Icons { none: "",      oct: "\u{f418} ", fa: "\u{f126} " };
pub const CWD_ICONS: Icons         = Icons { none: "cwd:", oct: "\u{f489} ", fa: "\u{f120} " };
pub const PROJECT_ICONS: Icons     = Icons { none: "proj:", oct: "\u{f46d} ", fa: "\u{f015} " };
pub const STYLE_ICONS: Icons       = Icons { none: "",     oct: "\u{f48f} ", fa: "\u{f1fc} " };
pub const SESSION_ID_ICONS: Icons  = Icons { none: "",     oct: "\u{f4ff} ", fa: "\u{f2c2} " };
pub const ALERT_ICONS: Icons       = Icons { none: "",     oct: "\u{f421} ", fa: "\u{f071} " };
pub const COST_VS_AVG_ICONS: Icons = Icons { none: "",     oct: "\u{f4a8} ", fa: "\u{f080} " };
pub const BURN_RATE_ICONS: Icons  = Icons { none: "",     oct: "\u{f490} ", fa: "\u{f06d} " };

impl Element {
    pub fn description(&self) -> &'static str {
        match self {
            Element::Model => "Model name (e.g. Opus 4.6)",
            Element::Version => "Claude Code version",
            Element::Context => "Context usage bar + percentage",
            Element::Tokens => "Input/output token counts",
            Element::Cache => "Cache read/write token counts",
            Element::Cost => "Session cost in USD",
            Element::Lines => "Lines added/removed",
            Element::Duration => "API wait time",
            Element::WallTime => "Wall clock elapsed time",
            Element::GitBranch => "Git branch name",
            Element::Cwd => "Current working directory",
            Element::ProjectDir => "Project root directory",
            Element::OutputStyle => "Output style indicator",
            Element::Alert => "Conditional alert badges",
            Element::SessionId => "Session identifier",
            Element::ProjectTodayCost => "Today's spend for current project",
            Element::BurnRate => "Cost per API hour",
            Element::SpendRate => "Cost per wall-clock hour",
            Element::DailyBudget => "Daily spend limit with progress bar",
            Element::SessionTokPerDollar => "Output tokens per dollar",
            Element::CacheHitRate => "Cache hit percentage",
            Element::CostVsAvg => "Cost vs other projects today",
            Element::CtxTrend => "Context usage direction",
            Element::AvgDailyCost => "Average daily spend",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Element::Model => "model",
            Element::Version => "version",
            Element::Context => "context",
            Element::Tokens => "tokens",
            Element::Cache => "cache",
            Element::Cost => "cost",
            Element::Lines => "lines",
            Element::Duration => "duration",
            Element::WallTime => "wall_time",
            Element::GitBranch => "git_branch",
            Element::Cwd => "cwd",
            Element::ProjectDir => "project",
            Element::OutputStyle => "style",
            Element::Alert => "alert",
            Element::ProjectTodayCost => "project_today_cost",
            Element::BurnRate => "burn_rate",
            Element::SpendRate => "spend_rate",
            Element::DailyBudget => "daily_budget",
            Element::SessionTokPerDollar => "session_tok_per_dollar",
            Element::CacheHitRate => "cache_hit_rate",
            Element::CostVsAvg => "cost_vs_avg",
            Element::CtxTrend => "ctx_trend",
            Element::AvgDailyCost => "avg_daily_cost",
            Element::SessionId => "session_id",
        }
    }
}

const ALL_ELEMENTS: &[Element] = &[
    Element::Model,
    Element::Version,
    Element::Context,
    Element::Tokens,
    Element::Cache,
    Element::Cost,
    Element::Lines,
    Element::Duration,
    Element::WallTime,
    Element::GitBranch,
    Element::Cwd,
    Element::ProjectDir,
    Element::OutputStyle,
    Element::Alert,
    Element::SessionId,
    Element::ProjectTodayCost,
    Element::BurnRate,
    Element::SpendRate,
    Element::DailyBudget,
    Element::SessionTokPerDollar,
    Element::CacheHitRate,
    Element::CostVsAvg,
    Element::CtxTrend,
    Element::AvgDailyCost,
];

pub const CORE_ELEMENT_NAMES: &[&str] = &[
    "model", "version", "context", "tokens", "cache",
    "cost", "lines", "duration", "wall_time", "git_branch", "cwd", "project", "style", "alert",
    "session_id",
];

pub const STATS_ELEMENT_NAMES: &[&str] = &[
    "project_today_cost", "burn_rate", "spend_rate",
    "daily_budget", "session_tok_per_dollar", "cache_hit_rate", "cost_vs_avg", "ctx_trend",
    "avg_daily_cost",
];

pub const ALL_ELEMENT_NAMES: &[&str] = &[
    "model", "version", "context", "tokens", "cache",
    "cost", "lines", "duration", "wall_time", "git_branch", "cwd", "project", "style", "alert",
    "session_id",
    "project_today_cost", "burn_rate", "spend_rate",
    "daily_budget", "session_tok_per_dollar", "cache_hit_rate", "cost_vs_avg", "ctx_trend",
    "avg_daily_cost",
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
        help = "Comma-separated elements; use --- for line break (model, version, context/ctx, tokens, cache, cost, lines, duration/time, wall_time/wall/elapsed, git_branch/branch/git, cwd, project/project_dir, style/output_style, alert, project_today_cost/daily_cost, burn_rate, spend_rate, daily_budget, session_tok_per_dollar, cache_hit_rate/cache_hit, cost_vs_avg, ctx_trend, avg_daily_cost)"
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

    #[arg(long, help = "Interactive configuration wizard")]
    pub configure: bool,

    #[arg(
        long,
        value_name = "SHELL",
        help = "Generate shell completions (bash, zsh, fish, elvish, powershell)"
    )]
    pub completions: Option<String>,

    #[arg(long, help = "Path to TOML config file")]
    pub config: Option<std::path::PathBuf>,

    #[arg(long, help = "Print default TOML config to stdout")]
    pub print_config: bool,

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

pub(crate) fn preset_elements(name: &str) -> Option<Vec<Vec<Element>>> {
    Some(match name {
        "minimal" => vec![vec![Element::Model, Element::Context, Element::Alert]],
        "compact" => vec![vec![
            Element::ProjectDir,
            Element::Model,
            Element::Context,
            Element::Cost,
            Element::Alert,
        ]],
        "default" => vec![vec![
            Element::ProjectDir,
            Element::Model,
            Element::OutputStyle,
            Element::GitBranch,
            Element::Context,
            Element::Cost,
            Element::Duration,
            Element::Alert,
        ]],
        "full" => vec![
            vec![
                Element::ProjectDir, Element::Model, Element::Version,
                Element::OutputStyle, Element::GitBranch, Element::Cwd,
                Element::DailyBudget, Element::SessionId,
            ],
            vec![
                Element::Context, Element::CtxTrend, Element::Cost,
                Element::WallTime, Element::SpendRate, Element::Duration,
                Element::BurnRate,
            ],
            vec![
                Element::Lines, Element::Tokens, Element::SessionTokPerDollar,
                Element::Cache, Element::CacheHitRate,
            ],
            vec![
                Element::ProjectTodayCost, Element::CostVsAvg,
                Element::AvgDailyCost,
            ],
            vec![Element::Alert],
        ],
        _ => return None,
    })
}

pub(crate) fn parse_element(s: &str) -> Option<Element> {
    match s.trim() {
        "model" => Some(Element::Model),
        "version" => Some(Element::Version),
        "context" | "ctx" => Some(Element::Context),
        "tokens" => Some(Element::Tokens),
        "cache" => Some(Element::Cache),
        "cost" => Some(Element::Cost),
        "lines" => Some(Element::Lines),
        "duration" | "time" => Some(Element::Duration),
        "wall_time" | "wall" | "elapsed" => Some(Element::WallTime),
        "git_branch" | "branch" | "git" => Some(Element::GitBranch),
        "cwd" => Some(Element::Cwd),
        "project" | "project_dir" => Some(Element::ProjectDir),
        "style" | "output_style" => Some(Element::OutputStyle),
        "alert" => Some(Element::Alert),
        "project_today_cost" | "daily_cost" => Some(Element::ProjectTodayCost),
        "burn_rate" => Some(Element::BurnRate),
        "spend_rate" => Some(Element::SpendRate),
        "daily_budget" => Some(Element::DailyBudget),
        "session_tok_per_dollar" => Some(Element::SessionTokPerDollar),
        "cache_hit_rate" | "cache_hit" => Some(Element::CacheHitRate),
        "cost_vs_avg" => Some(Element::CostVsAvg),
        "ctx_trend" => Some(Element::CtxTrend),
        "avg_daily_cost" => Some(Element::AvgDailyCost),
        "session_id" | "session" => Some(Element::SessionId),
        _ => None,
    }
}

pub const LINE_BREAK: &str = "---";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarItem {
    Element(Element),
    LineBreak,
}

pub fn bar_items_to_lines(items: &[BarItem]) -> Vec<Vec<BarItem>> {
    let mut lines: Vec<Vec<BarItem>> = vec![Vec::new()];
    for item in items {
        match item {
            BarItem::LineBreak => lines.push(Vec::new()),
            _ => lines.last_mut().unwrap().push(*item),
        }
    }
    lines.retain(|l| !l.is_empty());
    lines
}

pub fn element_lines_to_bar_items(lines: Vec<Vec<Element>>) -> Vec<BarItem> {
    let mut items = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for elem in line {
            items.push(BarItem::Element(*elem));
        }
        if i < lines.len() - 1 {
            items.push(BarItem::LineBreak);
        }
    }
    items
}

pub fn bar_items_from_layout(entries: &[String]) -> Vec<BarItem> {
    let mut items = Vec::new();
    for entry in entries {
        if entry.trim() == LINE_BREAK {
            items.push(BarItem::LineBreak);
        } else if let Some(elem) = parse_element(entry) {
            items.push(BarItem::Element(elem));
        }
    }
    items
}

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
        return preset_elements(name).unwrap_or_else(|| {
            eprintln!("Unknown preset: {name}. Use --info to see available presets.");
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
        return split_into_lines(layout.iter().map(|s| s.as_str()));
    }
    debug("elements: using built-in default preset");
    preset_elements("default").unwrap()
}

pub(crate) fn icon_mode_from_str(s: Option<&str>) -> IconMode {
    match s {
        Some("none" | "off") => IconMode::None,
        Some("fontawesome" | "fa") => IconMode::FontAwesome,
        _ => IconMode::Octicons,
    }
}

pub fn resolve_icon_mode(cli: &Cli, toml_icon_set: Option<&str>) -> IconMode {
    if cli.no_icons {
        debug("icons: disabled via --no-icons");
        return IconMode::None;
    }
    let env_set = env("CLAUDE_BAR_ICON_SET");
    let set = cli.icon_set.as_deref()
        .or(env_set.as_deref())
        .or(toml_icon_set);
    let mode = icon_mode_from_str(set);
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
  compact        project, model, context, cost, alert
  default        project, model, style, git_branch, context, cost, duration, alert
  full           all elements (5 lines)

ELEMENTS
  model          Model display name (e.g. Opus 4.6)
  version        Claude Code version
  context, ctx   Context usage bar + percentage (configurable)
  tokens         Input/output token counts
  cache          Cache read/write token counts
  cost           Session cost in USD
  lines          Lines added/removed this session
  duration, time API wait time
  wall_time,     Wall clock (elapsed) time
    wall, elapsed
  git_branch,    Git branch name (reads .git/HEAD)
    branch, git
  cwd            Current working directory (shortened)
  project,       Project root directory (shortened)
    project_dir
  style,         Output style (hidden when \"default\")
    output_style
  alert          Conditional badges (ctx exceeded, ctx high, budget)
  session_id,    Session identifier (first 8 characters)
    session

STATS ELEMENTS (require [stats] enabled = true)
  project_today_cost  Today's spend for current project (alias: daily_cost)
  burn_rate      Cost per API hour
  spend_rate     Cost per wall-clock hour
  daily_budget   Daily spend limit with progress bar
  session_tok_per_dollar  Output tokens per dollar (current session)
  cost_vs_avg    Current project cost vs other projects today
  ctx_trend      Context usage direction (configurable lookback)
  avg_daily_cost Average daily spend for current project (configurable lookback)

INPUT-ONLY ELEMENTS
  cache_hit_rate Cache hit percentage (no stats required)
    cache_hit

BAR STYLES (for context element)
  braille        Sub-cell braille dots (default, highest resolution)
  block          Filled/empty blocks (▰▱)
  shade          Gradient shading (█▓▒░)
  ascii          Plain ASCII ([###-------])
  progress       Nerd Font progress glyphs ()

ICON SETS
  octicons       Octicons (default)
  fontawesome    Font Awesome (alias: fa)
  none, off      No icons (text prefixes for paths)

MULTI-LINE LAYOUT
  Use \"---\" in layout.elements to split into multiple lines.
  Example: elements = [\"model\", \"context\", \"---\", \"cost\", \"duration\"]
  With --elements: --elements model,context,---,cost,duration

TIP: Run --configure for an interactive TUI to build your layout visually.
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
            configure: false,
            completions: None,
            print_config: false,
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
        let lines = preset_elements("minimal").unwrap();
        assert_eq!(lines.len(), 1);
        let elements = &lines[0];
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0], Element::Model);
        assert_eq!(elements[1], Element::Context);
        assert_eq!(elements[2], Element::Alert);
    }

    #[test]
    fn test_preset_elements_compact() {
        let lines = preset_elements("compact").unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            vec![Element::ProjectDir, Element::Model, Element::Context, Element::Cost, Element::Alert]
        );
    }

    #[test]
    fn test_preset_elements_default() {
        let lines = preset_elements("default").unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            vec![
                Element::ProjectDir, Element::Model, Element::OutputStyle, Element::GitBranch,
                Element::Context, Element::Cost, Element::Duration, Element::Alert,
            ]
        );
    }

    #[test]
    fn test_preset_elements_full() {
        let lines = preset_elements("full").unwrap();
        assert_eq!(lines.len(), 5);
        let all: Vec<Element> = lines.into_iter().flatten().collect();
        assert_eq!(all.len(), 24);
        for elem in ALL_ELEMENTS.iter() {
            assert!(all.contains(elem));
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
            "project_today_cost,burn_rate,spend_rate,daily_budget,session_tok_per_dollar,cache_hit_rate,cost_vs_avg,ctx_trend",
        );
        let line = &result[0];
        assert_eq!(line.len(), 8);
        assert!(line.contains(&Element::ProjectTodayCost));
        assert!(line.contains(&Element::SpendRate));
        assert!(line.contains(&Element::CacheHitRate));
    }

    #[test]
    fn test_parse_elements_daily_cost_alias() {
        let result = parse_elements("daily_cost");
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], Element::ProjectTodayCost);
    }

    #[test]
    fn test_parse_elements_cache_hit_alias() {
        let result = parse_elements("cache_hit");
        assert_eq!(result[0].len(), 1);
        assert_eq!(result[0][0], Element::CacheHitRate);
    }

    #[test]
    fn test_preset_elements_full_includes_stats() {
        let all: Vec<Element> = preset_elements("full").unwrap().into_iter().flatten().collect();
        assert!(all.contains(&Element::ProjectTodayCost));
        assert!(all.contains(&Element::BurnRate));
        assert!(all.contains(&Element::SpendRate));
        assert!(all.contains(&Element::CacheHitRate));
        assert!(all.contains(&Element::CtxTrend));
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
        assert_eq!(resolve_icon_mode(&cli, None), IconMode::None);
    }

    #[test]
    fn test_resolve_icon_mode_fontawesome() {
        let mut cli = test_cli();
        cli.icon_set = Some("fa".into());
        assert_eq!(resolve_icon_mode(&cli, None), IconMode::FontAwesome);
    }

    #[test]
    fn test_resolve_icon_mode_octicons_default() {
        let cli = test_cli();
        assert_eq!(resolve_icon_mode(&cli, None), IconMode::Octicons);
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

    #[test]
    fn test_element_name_matches_all_element_names() {
        for (elem, name) in ALL_ELEMENTS.iter().zip(ALL_ELEMENT_NAMES.iter()) {
            assert_eq!(
                elem.name(),
                *name,
                "Element::name() for {elem:?} does not match ALL_ELEMENT_NAMES entry '{name}'"
            );
        }
    }
}
