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
