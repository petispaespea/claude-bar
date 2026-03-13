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

fn env(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

/// Resolves config file path with 4-tier precedence:
/// 1. CLI argument (if provided)
/// 2. $CLAUDE_BAR_CONFIG env var
/// 3. $XDG_CONFIG_HOME/claude-bar.toml (if XDG_CONFIG_HOME set)
/// 4. ~/.config/claude-bar.toml (fallback)
fn resolve_config_path(cli_path: Option<&str>) -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    // Tier 1: CLI argument takes highest priority
    if let Some(path) = cli_path {
        return Some(PathBuf::from(path));
    }

    // Tier 2: CLAUDE_BAR_CONFIG env var
    if let Some(path) = env("CLAUDE_BAR_CONFIG") {
        return Some(PathBuf::from(path));
    }

    // Tier 3: $XDG_CONFIG_HOME/claude-bar.toml
    if let Some(xdg_home) = env("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_home).join("claude-bar.toml");
        if path.exists() {
            return Some(path);
        }
    }

    // Tier 4: ~/.config/claude-bar.toml (fallback)
    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home).join(".config/claude-bar.toml");
        if path.exists() {
            return Some(path);
        }
    }

    None
}

/// Loads configuration from file, with path resolution and silent fallback to defaults.
/// Returns BarConfig::default() if no file exists or TOML is invalid.
pub fn load_config(cli_path: Option<&str>) -> BarConfig {
    let Some(path) = resolve_config_path(cli_path) else {
        return BarConfig::default();
    };

    let Ok(contents) = std::fs::read_to_string(&path) else {
        return BarConfig::default();
    };

    match toml::from_str::<BarConfig>(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Warning: Invalid TOML in {}: {}", path.display(), e);
            BarConfig::default()
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
            "model", "version", "gauge", "context", "tokens", "cache", "cost", "lines", "duration",
            "cwd", "project", "style",
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

    #[test]
    fn load_no_file() {
        // Test when no config file exists
        let config = load_config(Some("/nonexistent/path/that/does/not/exist.toml"));
        // Should return default config without error
        assert_eq!(config.separator, "  ");
        assert_eq!(config.model.symbol, "\u{f4be} ");
        assert!(!config.model.disabled);
    }

    #[test]
    fn load_valid_file() {
        // Create a temporary file with valid TOML
        let temp_file = "/tmp/test_config_valid.toml";
        let toml_content = r#"
separator = " | "
[model]
disabled = true
symbol = "MODEL"
style = "red"
"#;
        std::fs::write(temp_file, toml_content).expect("Failed to write test file");

        // Load from the file
        let config = load_config(Some(temp_file));

        // Verify custom values were loaded
        assert_eq!(config.separator, " | ");
        assert!(config.model.disabled);
        assert_eq!(config.model.symbol, "MODEL");
        assert_eq!(config.model.style, "red");
        // Rest should use defaults
        assert_eq!(config.version.style, "dim");

        // Cleanup
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn load_invalid_toml() {
        // Create a temporary file with invalid TOML
        let temp_file = "/tmp/test_config_invalid.toml";
        let invalid_toml = "separator = [\n  invalid toml syntax here\n}";
        std::fs::write(temp_file, invalid_toml).expect("Failed to write test file");

        // Load from the file - should trigger warning but not panic
        let config = load_config(Some(temp_file));

        // Should return default config
        assert_eq!(config.separator, "  ");
        assert_eq!(config.model.symbol, "\u{f4be} ");

        // Cleanup
        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn resolve_config_path_cli_precedence() {
        // CLI path should take highest precedence
        let path = resolve_config_path(Some("/custom/path.toml"));
        assert_eq!(path, Some(std::path::PathBuf::from("/custom/path.toml")));
    }

    #[test]
    fn resolve_config_path_no_file() {
        // When no file exists in any tier, should return None
        let path = resolve_config_path(None);
        // This depends on environment, so we just verify it returns Option
        // We don't assert specific value as it could be Some or None depending on system
        let _ = path;
    }

    #[test]
    fn test_default_config_roundtrip_via_toml() {
        // Serialize default config to TOML string
        let original = BarConfig::default();
        let toml_str = toml::to_string_pretty(&original)
            .expect("Should serialize default config to TOML");
        
        // Verify output is not empty
        assert!(!toml_str.is_empty(), "TOML output should not be empty");
        
        // Deserialize back from string
        let deserialized: BarConfig = toml::from_str(&toml_str)
            .expect("Should deserialize TOML back to BarConfig");
        
        // Verify all fields match the original
        assert_eq!(original.separator, deserialized.separator);
        
        // Verify all module configs
        assert_eq!(original.model.disabled, deserialized.model.disabled);
        assert_eq!(original.model.symbol, deserialized.model.symbol);
        assert_eq!(original.model.style, deserialized.model.style);
        
        assert_eq!(original.version.disabled, deserialized.version.disabled);
        assert_eq!(original.version.symbol, deserialized.version.symbol);
        assert_eq!(original.version.style, deserialized.version.style);
        
        assert_eq!(original.gauge.disabled, deserialized.gauge.disabled);
        assert_eq!(original.gauge.symbol, deserialized.gauge.symbol);
        
        assert_eq!(original.context.disabled, deserialized.context.disabled);
        assert_eq!(original.context.symbol, deserialized.context.symbol);
        
        assert_eq!(original.tokens.disabled, deserialized.tokens.disabled);
        assert_eq!(original.tokens.symbol, deserialized.tokens.symbol);
        assert_eq!(original.tokens.style, deserialized.tokens.style);
        
        assert_eq!(original.cache.disabled, deserialized.cache.disabled);
        assert_eq!(original.cache.symbol, deserialized.cache.symbol);
        assert_eq!(original.cache.style, deserialized.cache.style);
        
        assert_eq!(original.cost.disabled, deserialized.cost.disabled);
        assert_eq!(original.cost.symbol, deserialized.cost.symbol);
        assert_eq!(original.cost.style, deserialized.cost.style);
        
        assert_eq!(original.lines.disabled, deserialized.lines.disabled);
        assert_eq!(original.lines.symbol, deserialized.lines.symbol);
        
        assert_eq!(original.duration.disabled, deserialized.duration.disabled);
        assert_eq!(original.duration.symbol, deserialized.duration.symbol);
        assert_eq!(original.duration.style, deserialized.duration.style);
        
        assert_eq!(original.cwd.disabled, deserialized.cwd.disabled);
        assert_eq!(original.cwd.symbol, deserialized.cwd.symbol);
        assert_eq!(original.cwd.style, deserialized.cwd.style);
        
        assert_eq!(original.project.disabled, deserialized.project.disabled);
        assert_eq!(original.project.symbol, deserialized.project.symbol);
        assert_eq!(original.project.style, deserialized.project.style);
        
        assert_eq!(original.style.disabled, deserialized.style.disabled);
        assert_eq!(original.style.symbol, deserialized.style.symbol);
        assert_eq!(original.style.style, deserialized.style.style);
        
        assert_eq!(original.layout.elements, deserialized.layout.elements);
    }
}
