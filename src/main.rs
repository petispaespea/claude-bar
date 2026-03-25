mod config;
mod configure;
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

    if cli.configure {
        configure::run();
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
    let bar_lines: Vec<Vec<config::BarItem>> = elements
        .into_iter()
        .map(|line| line.into_iter().map(config::BarItem::Element).collect())
        .collect();
    let icon_mode = config::resolve_icon_mode(&cli, config.as_ref().and_then(|c| c.icon_set.as_deref()));
    let config = config.unwrap_or_default();

    let mut input: input::Input = if cli.demo {
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

    if input.git_branch.is_none() {
        if let Some(ref cwd) = input.cwd {
            input.git_branch = git::branch(cwd);
        }
    }

    if config.stats.enabled && !cli.demo {
        stats::append_record(&input);
    }

    let agg_stats = if config.stats.enabled {
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
        Some(stats::compute_aggregate_stats(
            &stats::AggregateParams {
                today_records: &today,
                current_session_id: input.session_id.as_deref(),
                current_cost,
                current_api_ms,
                current_wall_ms,
                current_out_tok,
                budget_limit,
                current_project,
                ctx_lookback_secs: config.ctx_trend.lookback_secs,
            },
        ))
        .map(|mut s| {
            if let Some(proj) = current_project {
                s.avg_daily_cost = stats::compute_avg_daily_cost(proj, config.avg_daily_cost.lookback_days);
            }
            s
        })
    } else {
        None
    };

    let out = render::render_all(&bar_lines, &input, icon_mode, &config, &agg_stats);

    print!("{out}");
}
