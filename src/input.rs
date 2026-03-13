use serde::Deserialize;

#[derive(Deserialize)]
pub struct Input {
    pub model: Option<Model>,
    pub context_window: Option<ContextWindow>,
    pub cost: Option<Cost>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    pub exceeds_200k_tokens: Option<bool>,
    pub output_style: Option<OutputStyle>,
    pub workspace: Option<Workspace>,
}

#[derive(Deserialize)]
pub struct Model {
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ContextWindow {
    pub used_percentage: Option<f64>,
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Deserialize)]
pub struct CurrentUsage {
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Deserialize)]
pub struct Cost {
    pub total_cost_usd: Option<f64>,
    pub total_lines_added: Option<i64>,
    pub total_lines_removed: Option<i64>,
    pub total_api_duration_ms: Option<u64>,
}

#[derive(Deserialize)]
pub struct OutputStyle {
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub project_dir: Option<String>,
}

pub fn demo() -> Input {
    Input {
        model: Some(Model { display_name: Some("Opus 4.6".into()) }),
        context_window: Some(ContextWindow {
            used_percentage: Some(30.0),
            total_input_tokens: Some(3931),
            total_output_tokens: Some(28564),
            current_usage: Some(CurrentUsage {
                cache_creation_input_tokens: Some(1505),
                cache_read_input_tokens: Some(58984),
            }),
        }),
        cost: Some(Cost {
            total_cost_usd: Some(4.11),
            total_lines_added: Some(438),
            total_lines_removed: Some(265),
            total_api_duration_ms: Some(1_019_272),
        }),
        cwd: Some("/Users/demo/Git/my-project".into()),
        version: Some("2.1.69".into()),
        exceeds_200k_tokens: Some(false),
        output_style: Some(OutputStyle { name: Some("default".into()) }),
        workspace: Some(Workspace { project_dir: Some("/Users/demo/Git/my-project".into()) }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_demo_input_structure() {
        let input = demo();
        assert!(input.model.is_some());
        assert!(input.context_window.is_some());
        assert!(input.cost.is_some());
        assert!(input.cwd.is_some());
        assert!(input.version.is_some());
        assert!(input.exceeds_200k_tokens.is_some());
        assert!(input.output_style.is_some());
        assert!(input.workspace.is_some());
    }

    #[test]
    fn test_demo_model_display_name() {
        let input = demo();
        let model = input.model.unwrap();
        assert_eq!(model.display_name, Some("Opus 4.6".into()));
    }

    #[test]
    fn test_demo_cost_values() {
        let input = demo();
        let cost = input.cost.unwrap();
        assert_eq!(cost.total_cost_usd, Some(4.11));
        assert_eq!(cost.total_lines_added, Some(438));
        assert_eq!(cost.total_lines_removed, Some(265));
        assert_eq!(cost.total_api_duration_ms, Some(1_019_272));
    }

    #[test]
    fn test_deserialize_full_json_from_demo_status() {
        let json_str = r#"{
            "model": { "display_name": "Opus 4.6" },
            "context_window": {
                "used_percentage": 30.0,
                "total_input_tokens": 3931,
                "total_output_tokens": 28564,
                "current_usage": {
                    "cache_creation_input_tokens": 1505,
                    "cache_read_input_tokens": 58984
                }
            },
            "cost": {
                "total_cost_usd": 4.11,
                "total_lines_added": 438,
                "total_lines_removed": 265,
                "total_api_duration_ms": 1019272
            },
            "cwd": "/Users/demo/Git/my-project",
            "version": "2.1.69",
            "exceeds_200k_tokens": false,
            "output_style": { "name": "default" },
            "workspace": { "project_dir": "/Users/demo/Git/my-project" }
        }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        assert_eq!(input.version, Some("2.1.69".into()));
        assert_eq!(input.cwd, Some("/Users/demo/Git/my-project".into()));
    }

    #[test]
    fn test_deserialize_empty_json() {
        let json_str = "{}";
        let input: Input = serde_json::from_str(json_str).unwrap();
        assert!(input.model.is_none());
        assert!(input.context_window.is_none());
        assert!(input.cost.is_none());
        assert!(input.cwd.is_none());
        assert!(input.version.is_none());
        assert!(input.exceeds_200k_tokens.is_none());
        assert!(input.output_style.is_none());
        assert!(input.workspace.is_none());
    }

    #[test]
    fn test_deserialize_partial_json_single_field() {
        let json_str = r#"{ "version": "2.0.0" }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        assert_eq!(input.version, Some("2.0.0".into()));
        assert!(input.model.is_none());
        assert!(input.cost.is_none());
    }

    #[test]
    fn test_deserialize_partial_json_multiple_fields() {
        let json_str = r#"{
            "model": { "display_name": "Claude 3" },
            "cwd": "/home/user",
            "version": "1.5.0"
        }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        assert_eq!(input.version, Some("1.5.0".into()));
        assert_eq!(input.cwd, Some("/home/user".into()));
        assert!(input.model.is_some());
        assert!(input.cost.is_none());
    }

    #[test]
    fn test_deserialize_partial_context_window() {
        let json_str = r#"{
            "context_window": {
                "used_percentage": 50.0
            }
        }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        let ctx = input.context_window.unwrap();
        assert_eq!(ctx.used_percentage, Some(50.0));
        assert!(ctx.total_input_tokens.is_none());
        assert!(ctx.total_output_tokens.is_none());
        assert!(ctx.current_usage.is_none());
    }

    #[test]
    fn test_deserialize_nested_current_usage() {
        let json_str = r#"{
            "context_window": {
                "current_usage": {
                    "cache_read_input_tokens": 1000
                }
            }
        }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        let ctx = input.context_window.unwrap();
        let usage = ctx.current_usage.unwrap();
        assert_eq!(usage.cache_read_input_tokens, Some(1000));
        assert!(usage.cache_creation_input_tokens.is_none());
    }
}
