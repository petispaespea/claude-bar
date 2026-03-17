mod config;
mod format;
mod git;
mod input;
mod render;
mod setup;
mod stats;
mod style;
mod toml_config;

use clap::Parser;
use config::Cli;
use std::io::Read;

fn main() {
    let cli = Cli::parse();

    if let Some(ref shell) = cli.completions {
        let shell = shell.parse::<clap_complete::Shell>().unwrap_or_else(|_| {
            eprintln!("Unknown shell: {shell}. Use bash, zsh, fish, elvish or powershell.");
            std::process::exit(1);
        });
        clap_complete::generate(
            shell,
            &mut config::build_cli(),
            "claude-bar",
            &mut std::io::stdout(),
        );
        return;
    }

    if cli.setup {
        setup::run();
        return;
    }

    if cli.info {
        config::print_info();
        return;
    }

    if cli.print_config {
        print!("{}", toml_config::config_toml());
        return;
    }

    if cli.stats_clear {
        stats::clear_stats(cli.yes);
        return;
    }

    if cli.stats {
        let records = stats::load_records(cli.stats_days, cli.stats_project.as_deref());
        stats::print_summary(&records, cli.stats_days);
        return;
    }

    let config = toml_config::load_config(cli.config.as_ref().map(|p| p.to_str().unwrap()));
    let toml_layout = config.as_ref().map(|c| c.layout.elements.as_slice());
    let elements = config::resolve_elements(&cli, toml_layout);
    let icon_mode = config::resolve_icon_mode(&cli);
    let config = config.unwrap_or_default();

    let input: input::Input = if cli.demo {
        input::demo()
    } else {
        let mut buf = String::new();
        if std::io::stdin().read_to_string(&mut buf).is_err() {
            return;
        }
        match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(_) => return,
        }
    };

    if config.stats.enabled && !cli.demo {
        stats::append_record(&input);
    }

    let today_stats = if config.stats.enabled {
        let today = stats::load_today_records(&config.stats.day_window);
        let current_cost = input.cost.as_ref().and_then(|c| c.total_cost_usd);
        let current_api_ms = input.cost.as_ref().and_then(|c| c.total_api_duration_ms);
        let current_wall_ms = input.cost.as_ref().and_then(|c| c.total_duration_ms);
        let current_out_tok = input.context_window.as_ref().and_then(|c| c.total_output_tokens);
        let budget_limit = if config.daily_budget.limit > 0.0 {
            Some(config.daily_budget.limit)
        } else {
            None
        };
        let current_project = input.workspace.as_ref().and_then(|w| w.project_dir.as_deref());
        Some(stats::compute_today_stats(
            &today,
            input.session_id.as_deref(),
            current_cost,
            current_api_ms,
            current_wall_ms,
            current_out_tok,
            budget_limit,
            current_project,
        ))
    } else {
        None
    };

    let out: String = elements
        .iter()
        .map(|line| {
            line.iter()
                .filter_map(|e| render::render(*e, &input, icon_mode, &config, &today_stats))
                .collect::<Vec<_>>()
                .join(&config.separator)
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    print!("{out}");
}
