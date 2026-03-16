use serde::{Deserialize, Serialize};

use crate::config::{
    debug, env, CACHE_ICONS, CONTEXT_ICONS, COST_ICONS, COST_VS_AVG_ICONS, CWD_ICONS,
    DURATION_ICONS, LINES_ICONS, MODEL_ICONS, PROJECT_ICONS, SESSION_CT_ICONS, STYLE_ICONS,
    TOKENS_ICONS, VERSION_ICONS, WALL_TIME_ICONS,
};

macro_rules! module_config {
    ($name:ident, $icons:expr, $style:expr) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(default)]
        pub struct $name {
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

module_config!(ModelConfig,       MODEL_ICONS,    "cyan");
module_config!(VersionConfig,     VERSION_ICONS,  "dim");
module_config!(TokensConfig,      TOKENS_ICONS,   "dim");
module_config!(CacheConfig,       CACHE_ICONS,    "dim");
module_config!(CostConfig,        COST_ICONS,     "dim");
module_config!(LinesConfig,       LINES_ICONS,    "");
module_config!(DurationConfig,    DURATION_ICONS,  "dim");
module_config!(WallTimeConfig,    WALL_TIME_ICONS, "dim");
module_config!(CwdConfig,         CWD_ICONS,      "dim");
module_config!(ProjectDirConfig,  PROJECT_ICONS,   "dim");
module_config!(OutputStyleConfig, STYLE_ICONS,     "dim");
module_config!(DailyCostConfig,   COST_ICONS,       "");
module_config!(BurnRateConfig,    DURATION_ICONS,   "dim");
module_config!(SpendRateConfig,   DURATION_ICONS,   "dim");
module_config!(SessionCountConfig, SESSION_CT_ICONS, "dim");
module_config!(TokPerDollarConfig, TOKENS_ICONS,     "dim");
module_config!(CacheHitRateConfig, CACHE_ICONS,      "dim");
module_config!(CostVsAvgConfig,   COST_VS_AVG_ICONS, "dim");
module_config!(CtxTrendConfig,     CONTEXT_ICONS,    "");

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContextConfig {
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
}

impl Default for StatsConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct DailyBudgetConfig {
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
    #[serde(default = "AlertRule::default_label")]
    pub label: String,
    #[serde(default = "AlertRule::default_severity")]
    pub severity: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

impl AlertRule {
    fn default_label() -> String { "CTX EXCEEDED".into() }
    fn default_severity() -> String { "error".into() }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig {
    pub separator: String,
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
    pub cwd: CwdConfig,
    pub project: ProjectDirConfig,
    pub style: OutputStyleConfig,
    pub daily_cost: DailyCostConfig,
    pub burn_rate: BurnRateConfig,
    pub spend_rate: SpendRateConfig,
    pub session_count: SessionCountConfig,
    pub daily_budget: DailyBudgetConfig,
    pub tok_per_dollar: TokPerDollarConfig,
    pub cache_hit_rate: CacheHitRateConfig,
    pub cost_vs_avg: CostVsAvgConfig,
    pub ctx_trend: CtxTrendConfig,
    #[serde(rename = "alert")]
    pub alerts: Vec<AlertRule>,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            separator: " | ".into(),
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
            cwd: Default::default(),
            project: Default::default(),
            style: Default::default(),
            daily_cost: Default::default(),
            burn_rate: Default::default(),
            spend_rate: Default::default(),
            session_count: Default::default(),
            daily_budget: Default::default(),
            tok_per_dollar: Default::default(),
            cache_hit_rate: Default::default(),
            cost_vs_avg: Default::default(),
            ctx_trend: Default::default(),
            alerts: vec![
                AlertRule {
                    trigger: "ctx_exceeded".into(),
                    label: "CTX EXCEEDED".into(),
                    severity: "error".into(),
                    threshold: None,
                },
                AlertRule {
                    trigger: "cost_high".into(),
                    label: "BUDGET EXCEEDED".into(),
                    severity: "error".into(),
                    threshold: None,
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
        Self {
            elements: [
                "model", "version", "context", "tokens", "cache",
                "cost", "lines", "duration", "wall_time", "cwd", "project", "style", "alert",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
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
    let core = crate::config::CORE_ELEMENT_NAMES;
    let elements = core[..7].iter().map(|s| s.to_string())
        .chain(std::iter::once(brk.clone()))
        .chain(core[7..].iter().map(|s| s.to_string()))
        .chain(std::iter::once(brk))
        .chain(crate::config::STATS_ELEMENT_NAMES.iter().map(|s| s.to_string()))
        .collect();
    config.layout.elements = elements;
    let body = toml::to_string_pretty(&config).unwrap();
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
        assert_eq!(config.model.style, "cyan");
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
            "model", "version", "context", "tokens", "cache", "cost", "lines", "duration",
            "wall_time", "cwd", "project", "style", "alert",
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
        assert_eq!(config.cwd.symbol, "\u{f413} ");
        assert_eq!(config.project.symbol, "\u{f46d} ");
        assert_eq!(config.style.symbol, "\u{f48f} ");
    }

    #[test]
    fn test_dynamic_color_modules_have_empty_style() {
        let config = BarConfig::default();
        assert_eq!(config.context.style, "");
        assert_eq!(config.lines.style, "");
    }

    #[test]
    fn test_dim_modules_have_correct_style() {
        let config = BarConfig::default();
        assert_eq!(config.version.style, "dim");
        assert_eq!(config.tokens.style, "dim");
        assert_eq!(config.cache.style, "dim");
        assert_eq!(config.cost.style, "dim");
        assert_eq!(config.duration.style, "dim");
        assert_eq!(config.cwd.style, "dim");
        assert_eq!(config.project.style, "dim");
        assert_eq!(config.style.style, "dim");
    }

    #[test]
    fn test_model_is_cyan_only_colored_element() {
        let config = BarConfig::default();
        assert_eq!(config.model.style, "cyan");
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
        assert_eq!(config.alerts[1].trigger, "cost_high");
        assert_eq!(config.alerts[1].label, "BUDGET EXCEEDED");
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
        assert_eq!(config.alerts[1].trigger, "ctx_high");
        assert_eq!(config.alerts[1].threshold, Some(90.0));
    }
}
