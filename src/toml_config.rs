#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// Default value functions for serde
fn default_separator() -> String {
    "  ".to_string()
}

fn default_model_symbol() -> String {
    "\u{f4be} ".to_string()
}

fn default_model_style() -> String {
    "cyan".to_string()
}

fn default_version_symbol() -> String {
    "\u{f412} ".to_string()
}

fn default_version_style() -> String {
    "dim".to_string()
}

fn default_gauge_symbol() -> String {
    "\u{f4ed} ".to_string()
}

fn default_context_symbol() -> String {
    "\u{f463} ".to_string()
}

fn default_tokens_symbol() -> String {
    "\u{f4df} ".to_string()
}

fn default_tokens_style() -> String {
    "dim".to_string()
}

fn default_cache_symbol() -> String {
    "\u{f49b} ".to_string()
}

fn default_cache_style() -> String {
    "dim".to_string()
}

fn default_cost_symbol() -> String {
    "\u{f439} ".to_string()
}

fn default_cost_style() -> String {
    "dim".to_string()
}

fn default_lines_symbol() -> String {
    "\u{f4d2} ".to_string()
}

fn default_duration_symbol() -> String {
    "\u{f4e3} ".to_string()
}

fn default_duration_style() -> String {
    "dim".to_string()
}

fn default_cwd_symbol() -> String {
    "\u{f413} ".to_string()
}

fn default_cwd_style() -> String {
    "dim".to_string()
}

fn default_project_symbol() -> String {
    "\u{f46d} ".to_string()
}

fn default_project_style() -> String {
    "dim".to_string()
}

fn default_style_symbol() -> String {
    "\u{f48f} ".to_string()
}

fn default_style_style() -> String {
    "dim".to_string()
}

fn default_layout_elements() -> Vec<String> {
    vec![
        "model".to_string(),
        "version".to_string(),
        "gauge".to_string(),
        "context".to_string(),
        "tokens".to_string(),
        "cache".to_string(),
        "cost".to_string(),
        "lines".to_string(),
        "duration".to_string(),
        "cwd".to_string(),
        "project".to_string(),
        "style".to_string(),
    ]
}

/// Top-level configuration structure for the status bar.
/// All fields use serde(default) for backward compatibility.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BarConfig {
    #[serde(default = "default_separator")]
    pub separator: String,
    #[serde(default)]
    pub layout: LayoutConfig,
    #[serde(default)]
    pub model: ModelConfig,
    #[serde(default)]
    pub version: VersionConfig,
    #[serde(default)]
    pub gauge: GaugeConfig,
    #[serde(default)]
    pub context: ContextConfig,
    #[serde(default)]
    pub tokens: TokensConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub cost: CostConfig,
    #[serde(default)]
    pub lines: LinesConfig,
    #[serde(default)]
    pub duration: DurationConfig,
    #[serde(default)]
    pub cwd: CwdConfig,
    #[serde(default)]
    pub project: ProjectDirConfig,
    #[serde(default)]
    pub style: OutputStyleConfig,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            separator: default_separator(),
            layout: LayoutConfig::default(),
            model: ModelConfig::default(),
            version: VersionConfig::default(),
            gauge: GaugeConfig::default(),
            context: ContextConfig::default(),
            tokens: TokensConfig::default(),
            cache: CacheConfig::default(),
            cost: CostConfig::default(),
            lines: LinesConfig::default(),
            duration: DurationConfig::default(),
            cwd: CwdConfig::default(),
            project: ProjectDirConfig::default(),
            style: OutputStyleConfig::default(),
        }
    }
}

/// Layout configuration specifying which elements are enabled and their order.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayoutConfig {
    #[serde(default = "default_layout_elements")]
    pub elements: Vec<String>,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            elements: default_layout_elements(),
        }
    }
}

/// Model element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_model_symbol")]
    pub symbol: String,
    #[serde(default = "default_model_style")]
    pub style: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_model_symbol(),
            style: default_model_style(),
        }
    }
}

/// Version element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_version_symbol")]
    pub symbol: String,
    #[serde(default = "default_version_style")]
    pub style: String,
}

impl Default for VersionConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_version_symbol(),
            style: default_version_style(),
        }
    }
}

/// Gauge element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GaugeConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_gauge_symbol")]
    pub symbol: String,
    #[serde(default)]
    pub style: String,
}

impl Default for GaugeConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_gauge_symbol(),
            style: String::new(),
        }
    }
}

/// Context element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_context_symbol")]
    pub symbol: String,
    #[serde(default)]
    pub style: String,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_context_symbol(),
            style: String::new(),
        }
    }
}

/// Tokens element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokensConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_tokens_symbol")]
    pub symbol: String,
    #[serde(default = "default_tokens_style")]
    pub style: String,
}

impl Default for TokensConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_tokens_symbol(),
            style: default_tokens_style(),
        }
    }
}

/// Cache element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_cache_symbol")]
    pub symbol: String,
    #[serde(default = "default_cache_style")]
    pub style: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_cache_symbol(),
            style: default_cache_style(),
        }
    }
}

/// Cost element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CostConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_cost_symbol")]
    pub symbol: String,
    #[serde(default = "default_cost_style")]
    pub style: String,
}

impl Default for CostConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_cost_symbol(),
            style: default_cost_style(),
        }
    }
}

/// Lines element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinesConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_lines_symbol")]
    pub symbol: String,
    #[serde(default)]
    pub style: String,
}

impl Default for LinesConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_lines_symbol(),
            style: String::new(),
        }
    }
}

/// Duration element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DurationConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_duration_symbol")]
    pub symbol: String,
    #[serde(default = "default_duration_style")]
    pub style: String,
}

impl Default for DurationConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_duration_symbol(),
            style: default_duration_style(),
        }
    }
}

/// Current working directory element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CwdConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_cwd_symbol")]
    pub symbol: String,
    #[serde(default = "default_cwd_style")]
    pub style: String,
}

impl Default for CwdConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_cwd_symbol(),
            style: default_cwd_style(),
        }
    }
}

/// Project directory element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectDirConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_project_symbol")]
    pub symbol: String,
    #[serde(default = "default_project_style")]
    pub style: String,
}

impl Default for ProjectDirConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_project_symbol(),
            style: default_project_style(),
        }
    }
}

/// Output style element configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputStyleConfig {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default = "default_style_symbol")]
    pub symbol: String,
    #[serde(default = "default_style_style")]
    pub style: String,
}

impl Default for OutputStyleConfig {
    fn default() -> Self {
        Self {
            disabled: false,
            symbol: default_style_symbol(),
            style: default_style_style(),
        }
    }
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
        let empty_toml = "";
        let config: BarConfig = toml::from_str(empty_toml).expect("Should deserialize");
        assert_eq!(config.separator, "  ");
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.model.style, "cyan");
        assert!(!config.model.disabled);
    }

    #[test]
    fn test_deserialize_partial_override_with_rest_defaults() {
        let partial_toml = r#"
separator = "||"

[model]
disabled = true
"#;
        let config: BarConfig = toml::from_str(partial_toml).expect("Should deserialize");
        assert_eq!(config.separator, "||");
        assert!(config.model.disabled);
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.model.style, "cyan");
        assert!(!config.version.disabled);
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
            "model", "version", "gauge", "context", "tokens", "cache", "cost", "lines",
            "duration", "cwd", "project", "style",
        ];
        assert_eq!(config.layout.elements, expected);
    }

    #[test]
    fn test_all_module_configs_have_correct_symbols() {
        let config = BarConfig::default();
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert_eq!(config.version.symbol, "\u{f412} ");
        assert_eq!(config.gauge.symbol, "\u{f4ed} ");
        assert_eq!(config.context.symbol, "\u{f463} ");
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
        assert_eq!(config.gauge.style, "");
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
        assert_eq!(original.model.disabled, deserialized.model.disabled);
        assert_eq!(original.model.symbol, deserialized.model.symbol);
        assert_eq!(original.model.style, deserialized.model.style);
        assert_eq!(original.layout.elements, deserialized.layout.elements);
    }
}
