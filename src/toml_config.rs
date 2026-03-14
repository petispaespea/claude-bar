use serde::{Deserialize, Serialize};

macro_rules! module_config {
    ($name:ident, $symbol:expr, $style:expr) => {
        #[derive(Debug, Clone, Deserialize, Serialize)]
        #[serde(default)]
        pub struct $name {
            pub symbol: String,
            pub style: String,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    symbol: $symbol.to_string(),
                    style: $style.to_string(),
                }
            }
        }
    };
}

module_config!(ModelConfig,       "\u{f4be} ", "cyan");
module_config!(VersionConfig,     "\u{f412} ", "dim");
module_config!(GaugeConfig,       "\u{f4ed} ", "");
module_config!(ContextConfig,     "\u{f463} ", "");
module_config!(TokensConfig,      "\u{f4df} ", "dim");
module_config!(CacheConfig,       "\u{f49b} ", "dim");
module_config!(CostConfig,        "\u{f439} ", "dim");
module_config!(LinesConfig,       "\u{f4d2} ", "");
module_config!(DurationConfig,    "\u{f4e3} ", "dim");
module_config!(CwdConfig,         "\u{f413} ", "dim");
module_config!(ProjectDirConfig,  "\u{f46d} ", "dim");
module_config!(OutputStyleConfig, "\u{f48f} ", "dim");

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig {
    pub separator: String,
    pub layout: LayoutConfig,
    pub model: ModelConfig,
    pub version: VersionConfig,
    pub gauge: GaugeConfig,
    pub context: ContextConfig,
    pub tokens: TokensConfig,
    pub cache: CacheConfig,
    pub cost: CostConfig,
    pub lines: LinesConfig,
    pub duration: DurationConfig,
    pub cwd: CwdConfig,
    pub project: ProjectDirConfig,
    pub style: OutputStyleConfig,
}

impl Default for BarConfig {
    fn default() -> Self {
        Self {
            separator: "  ".into(),
            layout: Default::default(),
            model: Default::default(),
            version: Default::default(),
            gauge: Default::default(),
            context: Default::default(),
            tokens: Default::default(),
            cache: Default::default(),
            cost: Default::default(),
            lines: Default::default(),
            duration: Default::default(),
            cwd: Default::default(),
            project: Default::default(),
            style: Default::default(),
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
                "model", "version", "gauge", "context", "tokens", "cache",
                "cost", "lines", "duration", "cwd", "project", "style",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }
}

use crate::config::{debug, env};

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

pub fn default_config_toml() -> String {
    let header = "\
# claude-bar configuration
# Place at ~/.config/claude-bar.toml or set $CLAUDE_BAR_CONFIG.
#
# Elements are controlled via [layout]. Remove entries to hide them.
# Per-element options: symbol (Nerd Font icon prefix), style (ANSI attrs).
# Styles: black red green yellow blue magenta cyan white bold dim italic underline
# Empty style \"\" means the element controls its own color dynamically.
";
    let body = toml::to_string_pretty(&BarConfig::default()).unwrap();
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
        assert_eq!(config.separator, "  ");
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
        assert_eq!(original.gauge.symbol, deserialized.gauge.symbol);
        assert_eq!(original.context.symbol, deserialized.context.symbol);
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
    }
}
