use serde::{Deserialize, Serialize};

use crate::config::{
    debug, env, BURN_RATE_ICONS, CACHE_ICONS, CONTEXT_ICONS, COST_ICONS, COST_VS_AVG_ICONS,
    CWD_ICONS, DURATION_ICONS, GIT_BRANCH_ICONS, LINES_ICONS, MODEL_ICONS, PROJECT_ICONS,
    STYLE_ICONS, TOKENS_ICONS, VERSION_ICONS, WALL_TIME_ICONS,
};

macro_rules! module_config {
    ($name:ident, $icons:expr, $style:expr) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(default)]
        pub struct $name {
            #[serde(skip_serializing)]
            pub symbol: String,
            pub style: String,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    symbol: $icons.oct.to_string(),
                    style: $style.to_string(),
                }
            }
        }
    };
}

module_config!(ModelConfig,       MODEL_ICONS,    "magenta");
module_config!(VersionConfig,     VERSION_ICONS,  "dim");
module_config!(TokensConfig,      TOKENS_ICONS,   "dim");
module_config!(CacheConfig,       CACHE_ICONS,    "dim");
module_config!(CostConfig,        COST_ICONS,     "green");
module_config!(LinesConfig,       LINES_ICONS,    "");
module_config!(DurationConfig,    DURATION_ICONS,  "dim cyan");
module_config!(WallTimeConfig,    WALL_TIME_ICONS, "cyan");
module_config!(GitBranchConfig,   GIT_BRANCH_ICONS, "magenta");
module_config!(CwdConfig,         CWD_ICONS,      "dim blue");
module_config!(ProjectDirConfig,  PROJECT_ICONS,   "blue");
module_config!(OutputStyleConfig, STYLE_ICONS,     "dim magenta");
module_config!(ProjectTodayCostConfig, COST_ICONS, "blue");
module_config!(BurnRateConfig,    BURN_RATE_ICONS,  "dim cyan");
module_config!(SpendRateConfig,   COST_ICONS,   "cyan");
module_config!(SessionTokPerDollarConfig, TOKENS_ICONS,     "dim cyan");
module_config!(CacheHitRateConfig, CACHE_ICONS,      "cyan");
module_config!(CostVsAvgConfig,   COST_VS_AVG_ICONS, "dim blue");

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AvgDailyCostConfig {
    #[serde(skip_serializing)]
    pub symbol: String,
    pub style: String,
    pub lookback_days: u64,
}

impl Default for AvgDailyCostConfig {
    fn default() -> Self {
        Self {
            symbol: COST_ICONS.oct.to_string(),
            style: "dim blue".to_string(),
            lookback_days: 30,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CtxTrendConfig {
    #[serde(skip_serializing)]
    pub symbol: String,
    pub style: String,
    pub lookback_secs: u64,
}

impl Default for CtxTrendConfig {
    fn default() -> Self {
        Self {
            symbol: CONTEXT_ICONS.oct.to_string(),
            style: String::new(),
            lookback_secs: 300,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContextConfig {
    #[serde(skip_serializing)]
    pub symbol: String,
    pub style: String,
    pub bar_style: String,
    pub width: usize,
    pub show_bar: bool,
    pub show_pct: bool,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            symbol: CONTEXT_ICONS.oct.to_string(),
            style: String::new(),
            bar_style: "braille".to_string(),
            width: 10,
            show_bar: true,
            show_pct: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct StatsConfig {
    pub enabled: bool,
    pub day_window: String,
}

impl Default for StatsConfig {
    fn default() -> Self {
        Self { enabled: false, day_window: "calendar".to_string() }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct DailyBudgetConfig {
    #[serde(skip_serializing)]
    pub symbol: String,
    pub style: String,
    pub bar_style: String,
    pub width: usize,
    pub show_bar: bool,
    pub show_pct: bool,
    pub limit: f64,
}

impl Default for DailyBudgetConfig {
    fn default() -> Self {
        Self {
            symbol: COST_ICONS.oct.to_string(),
            style: String::new(),
            bar_style: "block".to_string(),
            width: 8,
            show_bar: true,
            show_pct: true,
            limit: 100.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlertRule {
    pub trigger: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default = "AlertRule::default_severity")]
    pub severity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl AlertRule {
    fn default_severity() -> String { "error".into() }
    pub fn display_label(&self) -> String {
        self.label.clone().unwrap_or_else(|| match self.trigger.as_str() {
            "ctx_exceeded" => "CTX EXCEEDED".into(),
            "ctx_high" => "CTX HIGH".into(),
            "cost_high" => "BUDGET EXCEEDED".into(),
            other => other.to_uppercase().replace('_', " "),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig {
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_set: Option<String>,
    pub layout: LayoutConfig,
    pub stats: StatsConfig,
    pub model: ModelConfig,
    pub version: VersionConfig,
    pub context: ContextConfig,
    pub tokens: TokensConfig,
    pub cache: CacheConfig,
    pub cost: CostConfig,
    pub lines: LinesConfig,
    pub duration: DurationConfig,
    pub wall_time: WallTimeConfig,
    pub git_branch: GitBranchConfig,
    pub cwd: CwdConfig,
    pub project: ProjectDirConfig,
    pub style: OutputStyleConfig,
    #[serde(alias = "daily_cost")]
    pub project_today_cost: ProjectTodayCostConfig,
    pub burn_rate: BurnRateConfig,
    pub spend_rate: SpendRateConfig,
    pub daily_budget: DailyBudgetConfig,
    pub session_tok_per_dollar: SessionTokPerDollarConfig,
    pub cache_hit_rate: CacheHitRateConfig,
    pub cost_vs_avg: CostVsAvgConfig,
    pub avg_daily_cost: AvgDailyCostConfig,
    pub ctx_trend: CtxTrendConfig,
    #[serde(rename = "alert")]
    pub alerts: Vec<AlertRule>,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            separator: " | ".into(),
            icon_set: None,
            layout: Default::default(),
            stats: Default::default(),
            model: Default::default(),
            version: Default::default(),
            context: Default::default(),
            tokens: Default::default(),
            cache: Default::default(),
            cost: Default::default(),
            lines: Default::default(),
            duration: Default::default(),
            wall_time: Default::default(),
            git_branch: Default::default(),
            cwd: Default::default(),
            project: Default::default(),
            style: Default::default(),
            project_today_cost: Default::default(),
            burn_rate: Default::default(),
            spend_rate: Default::default(),
            daily_budget: Default::default(),
            session_tok_per_dollar: Default::default(),
            cache_hit_rate: Default::default(),
            cost_vs_avg: Default::default(),
            avg_daily_cost: Default::default(),
            ctx_trend: Default::default(),
            alerts: vec![
                AlertRule {
                    trigger: "ctx_exceeded".into(),
                    label: None,
                    severity: "error".into(),
                    threshold: None,
                    symbol: None,
                },
                AlertRule {
                    trigger: "cost_high".into(),
                    label: None,
                    severity: "error".into(),
                    threshold: None,
                    symbol: None,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LayoutConfig {
    pub elements: Vec<String>,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        let elements = crate::config::preset_elements("default")
            .unwrap()
            .into_iter()
            .flatten()
            .map(|e| e.name().to_string())
            .collect();
        Self { elements }
    }
}

fn resolve_config_path(cli_path: Option<&str>) -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    if let Some(path) = cli_path {
        debug(&format!("config: using --config {path}"));
        return Some(PathBuf::from(path));
    }

    if let Some(path) = env("CLAUDE_BAR_CONFIG") {
        debug(&format!("config: using $CLAUDE_BAR_CONFIG={path}"));
        return Some(PathBuf::from(path));
    }

    if let Some(xdg_home) = env("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_home).join("claude-bar.toml");
        debug(&format!("config: trying {}", path.display()));
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home).join(".config/claude-bar.toml");
        debug(&format!("config: trying {}", path.display()));
        if path.exists() {
            return Some(path);
        }
    }

    debug("config: no config file found, using defaults");
    None
}

pub fn load_config(cli_path: Option<&str>) -> Option<BarConfig> {
    let path = resolve_config_path(cli_path)?;

    let Ok(contents) = std::fs::read_to_string(&path) else {
        debug(&format!("config: could not read {}, using defaults", path.display()));
        return None;
    };

    match toml::from_str::<BarConfig>(&contents) {
        Ok(config) => {
            debug(&format!("config: loaded {}", path.display()));
            Some(config)
        }
        Err(e) => {
            eprintln!("Warning: Invalid TOML in {}: {}", path.display(), e);
            None
        }
    }
}

pub fn config_toml() -> String {
    let header = "\
# claude-bar configuration
# Place at ~/.config/claude-bar.toml or set $CLAUDE_BAR_CONFIG.
#
# Elements are controlled via [layout]. Remove entries to hide them.
# Per-element options: symbol (Nerd Font icon prefix), style (ANSI attrs).
# Styles: black red green yellow blue magenta cyan white bold dim italic underline
# Empty style \"\" means the element controls its own color dynamically.
";
    let mut config = BarConfig::default();
    config.stats.enabled = true;
    let brk = crate::config::LINE_BREAK.to_string();
    let elements: Vec<String> =
        ["project", "model", "version", "style", "git_branch", "cwd", "daily_budget"].iter().map(|s| s.to_string())
        .chain(std::iter::once(brk.clone()))
        .chain(["context", "ctx_trend", "cost", "wall_time", "spend_rate", "duration", "burn_rate"].iter().map(|s| s.to_string()))
        .chain(std::iter::once(brk.clone()))
        .chain(["lines", "tokens", "session_tok_per_dollar", "cache", "cache_hit_rate"].iter().map(|s| s.to_string()))
        .chain(std::iter::once(brk.clone()))
        .chain(["project_today_cost", "cost_vs_avg", "avg_daily_cost"].iter().map(|s| s.to_string()))
        .chain(std::iter::once(brk))
        .chain(["alert"].iter().map(|s| s.to_string()))
        .collect();
    config.layout.elements = elements;
    let body = toml::to_string_pretty(&config).unwrap();
    let alert_comment = "\
# Specifying any [[alert]] replaces ALL defaults.\n\
# label is optional — derived from trigger if omitted.\n\
# cost_high requires [stats] enabled = true.\n";
    let body = body.replace("[[alert]]", &format!("{alert_comment}[[alert]]"));
    format!("{header}\n{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bar_config_serializes_to_toml() {
        let config = BarConfig::default();
        let toml_str = toml::to_string(&config).expect("Should serialize to TOML");
        assert!(!toml_str.is_empty());
        assert!(toml_str.contains("separator"));
    }

    #[test]
    fn test_deserialize_empty_string_uses_all_defaults() {
        let config: BarConfig = toml::from_str("").expect("Should deserialize");
        assert_eq!(config.separator, " | ");
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.model.style, "magenta");
    }

    #[test]
    fn test_deserialize_partial_override_with_rest_defaults() {
        let partial_toml = r#"
separator = "||"

[model]
style = "red"
"#;
        let config: BarConfig = toml::from_str(partial_toml).expect("Should deserialize");
        assert_eq!(config.separator, "||");
        assert_eq!(config.model.style, "red");
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.version.style, "dim");
    }

    #[test]
    fn test_deserialize_unknown_fields_silently_ignored() {
        let with_unknown = r#"
separator = "  "
unknown_field = "ignored"

[model]
symbol = "custom"
unknown_config = 123
"#;
        let config: BarConfig = toml::from_str(with_unknown).expect("Should deserialize");
        assert_eq!(config.separator, "  ");
        assert_eq!(config.model.symbol, "custom");
    }

    #[test]
    fn test_default_layout_matches_all_elements_order() {
        let config = BarConfig::default();
        let expected = vec![
            "model", "context", "cost", "duration", "git_branch", "project", "style", "alert",
        ];
        assert_eq!(config.layout.elements, expected);
    }

    #[test]
    fn test_all_module_configs_have_correct_symbols() {
        let config = BarConfig::default();
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.version.symbol, "\u{f412} ");
        assert_eq!(config.context.symbol, "\u{f4ed} ");
        assert_eq!(config.tokens.symbol, "\u{f4df} ");
        assert_eq!(config.cache.symbol, "\u{f49b} ");
        assert_eq!(config.cost.symbol, "\u{f439} ");
        assert_eq!(config.lines.symbol, "\u{f4d2} ");
        assert_eq!(config.duration.symbol, "\u{f4e3} ");
        assert_eq!(config.cwd.symbol, "\u{f489} ");
        assert_eq!(config.project.symbol, "\u{f46d} ");
        assert_eq!(config.style.symbol, "\u{f48f} ");
    }

    #[test]
    fn test_dynamic_color_modules_have_empty_style() {
        let config = BarConfig::default();
        assert_eq!(config.context.style, "");
        assert_eq!(config.lines.style, "");
        assert_eq!(config.ctx_trend.style, "");
    }

    #[test]
    fn test_semantic_styles() {
        let config = BarConfig::default();
        assert_eq!(config.model.style, "magenta");
        assert_eq!(config.version.style, "dim");
        assert_eq!(config.tokens.style, "dim");
        assert_eq!(config.cache.style, "dim");
        assert_eq!(config.cost.style, "green");
        assert_eq!(config.duration.style, "dim cyan");
        assert_eq!(config.wall_time.style, "cyan");
        assert_eq!(config.cwd.style, "dim blue");
        assert_eq!(config.project.style, "blue");
        assert_eq!(config.style.style, "dim magenta");
        assert_eq!(config.project_today_cost.style, "blue");
        assert_eq!(config.burn_rate.style, "dim cyan");
        assert_eq!(config.spend_rate.style, "cyan");
        assert_eq!(config.session_tok_per_dollar.style, "dim cyan");
        assert_eq!(config.cost_vs_avg.style, "dim blue");
        assert_eq!(config.cache_hit_rate.style, "cyan");
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = BarConfig::default();
        let toml_str = toml::to_string(&original).expect("Should serialize");
        let deserialized: BarConfig = toml::from_str(&toml_str).expect("Should deserialize");

        assert_eq!(original.separator, deserialized.separator);
        assert_eq!(original.model.symbol, deserialized.model.symbol);
        assert_eq!(original.model.style, deserialized.model.style);
        assert_eq!(original.layout.elements, deserialized.layout.elements);
    }

    #[test]
    fn load_no_file() {
        let config = load_config(Some("/nonexistent/path/that/does/not/exist.toml"));
        assert!(config.is_none());
    }

    #[test]
    fn load_valid_file() {
        let temp_file = "/tmp/test_config_valid.toml";
        let toml_content = r#"
separator = " | "
[model]
symbol = "MODEL"
style = "red"
"#;
        std::fs::write(temp_file, toml_content).expect("Failed to write test file");

        let config = load_config(Some(temp_file)).expect("Should load valid config");

        assert_eq!(config.separator, " | ");
        assert_eq!(config.model.symbol, "MODEL");
        assert_eq!(config.model.style, "red");
        assert_eq!(config.version.style, "dim");

        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn load_invalid_toml() {
        let temp_file = "/tmp/test_config_invalid.toml";
        let invalid_toml = "separator = [\n  invalid toml syntax here\n}";
        std::fs::write(temp_file, invalid_toml).expect("Failed to write test file");

        let config = load_config(Some(temp_file));
        assert!(config.is_none());

        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn resolve_config_path_cli_precedence() {
        let path = resolve_config_path(Some("/custom/path.toml"));
        assert_eq!(path, Some(std::path::PathBuf::from("/custom/path.toml")));
    }

    #[test]
    fn resolve_config_path_no_file() {
        let _ = resolve_config_path(None);
    }

    #[test]
    fn test_default_config_roundtrip_via_toml() {
        let original = BarConfig::default();
        let toml_str =
            toml::to_string_pretty(&original).expect("Should serialize default config to TOML");
        assert!(!toml_str.is_empty());

        let deserialized: BarConfig =
            toml::from_str(&toml_str).expect("Should deserialize TOML back to BarConfig");

        assert_eq!(original.separator, deserialized.separator);
        assert_eq!(original.model.symbol, deserialized.model.symbol);
        assert_eq!(original.model.style, deserialized.model.style);
        assert_eq!(original.version.symbol, deserialized.version.symbol);
        assert_eq!(original.version.style, deserialized.version.style);
        assert_eq!(original.context.symbol, deserialized.context.symbol);
        assert_eq!(original.context.bar_style, deserialized.context.bar_style);
        assert_eq!(original.context.width, deserialized.context.width);
        assert_eq!(original.context.show_bar, deserialized.context.show_bar);
        assert_eq!(original.context.show_pct, deserialized.context.show_pct);
        assert_eq!(original.tokens.symbol, deserialized.tokens.symbol);
        assert_eq!(original.tokens.style, deserialized.tokens.style);
        assert_eq!(original.cache.symbol, deserialized.cache.symbol);
        assert_eq!(original.cache.style, deserialized.cache.style);
        assert_eq!(original.cost.symbol, deserialized.cost.symbol);
        assert_eq!(original.cost.style, deserialized.cost.style);
        assert_eq!(original.lines.symbol, deserialized.lines.symbol);
        assert_eq!(original.duration.symbol, deserialized.duration.symbol);
        assert_eq!(original.duration.style, deserialized.duration.style);
        assert_eq!(original.cwd.symbol, deserialized.cwd.symbol);
        assert_eq!(original.cwd.style, deserialized.cwd.style);
        assert_eq!(original.project.symbol, deserialized.project.symbol);
        assert_eq!(original.project.style, deserialized.project.style);
        assert_eq!(original.style.symbol, deserialized.style.symbol);
        assert_eq!(original.style.style, deserialized.style.style);
        assert_eq!(original.layout.elements, deserialized.layout.elements);
        assert_eq!(original.alerts.len(), deserialized.alerts.len());
        assert_eq!(original.alerts[0].trigger, deserialized.alerts[0].trigger);
    }

    #[test]
    fn test_context_config_defaults() {
        let config = BarConfig::default();
        assert_eq!(config.context.bar_style, "braille");
        assert_eq!(config.context.width, 10);
        assert!(config.context.show_bar);
        assert!(config.context.show_pct);
    }

    #[test]
    fn test_context_config_partial_override() {
        let toml_str = r#"
[context]
bar_style = "block"
show_pct = false
"#;
        let config: BarConfig = toml::from_str(toml_str).expect("Should deserialize");
        assert_eq!(config.context.bar_style, "block");
        assert!(!config.context.show_pct);
        assert!(config.context.show_bar);
        assert_eq!(config.context.width, 10);
    }

    #[test]
    fn test_default_alert_rules() {
        let config = BarConfig::default();
        assert_eq!(config.alerts.len(), 2);
        assert_eq!(config.alerts[0].trigger, "ctx_exceeded");
        assert_eq!(config.alerts[0].severity, "error");
        assert!(config.alerts[0].threshold.is_none());
        assert!(config.alerts[0].label.is_none());
        assert_eq!(config.alerts[0].display_label(), "CTX EXCEEDED");
        assert_eq!(config.alerts[1].trigger, "cost_high");
        assert!(config.alerts[1].label.is_none());
        assert_eq!(config.alerts[1].display_label(), "BUDGET EXCEEDED");
    }

    #[test]
    fn test_alert_rules_deserialization() {
        let toml_str = r#"
[[alert]]
trigger = "ctx_exceeded"
label = "CTX EXCEEDED"
severity = "error"

[[alert]]
trigger = "ctx_high"
label = "CTX 90%+"
severity = "warn"
threshold = 90.0
"#;
        let config: BarConfig = toml::from_str(toml_str).expect("Should deserialize");
        assert_eq!(config.alerts.len(), 2);
        assert_eq!(config.alerts[0].trigger, "ctx_exceeded");
        assert_eq!(config.alerts[0].label, Some("CTX EXCEEDED".into()));
        assert_eq!(config.alerts[1].trigger, "ctx_high");
        assert_eq!(config.alerts[1].threshold, Some(90.0));
    }
}
