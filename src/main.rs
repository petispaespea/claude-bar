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

fn probe_parent_tty_width() -> Option<usize> {
    #[cfg(not(target_os = "macos"))]
    return None;

    #[cfg(target_os = "macos")]
    {
        let mut pid = unsafe { libc::getpid() };
        for _ in 0..16 {
            if pid <= 1 {
                break;
            }
            if let Some((tdev, ppid)) = sysctl_proc_info(pid) {
                if let Some(cols) = tty_width_from_dev(tdev) {
                    return Some(cols);
                }
                pid = ppid;
            } else {
                break;
            }
        }
        None
    }
}

/// Returns (e_tdev, e_ppid) for the given pid via sysctl.
#[cfg(target_os = "macos")]
fn sysctl_proc_info(pid: libc::pid_t) -> Option<(i32, i32)> {
    // Offsets into struct kinfo_proc (from <sys/sysctl.h>, macOS 15 SDK).
    // Layout is stable across arm64 and x86_64. Fields:
    //   kp_eproc.e_ppid  (pid_t)  at byte 560
    //   kp_eproc.e_tdev  (dev_t)  at byte 572
    const KINFO_SIZE: usize = 648;
    const PPID_OFF: usize = 560;
    const TDEV_OFF: usize = 572;

    let mut buf = [0u8; KINFO_SIZE];
    let mut size = KINFO_SIZE;
    let mut mib = [libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_PID, pid];
    let ret = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            4,
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if ret != 0 || size == 0 {
        return None;
    }
    let ppid = i32::from_ne_bytes(buf[PPID_OFF..PPID_OFF + 4].try_into().ok()?);
    let tdev = i32::from_ne_bytes(buf[TDEV_OFF..TDEV_OFF + 4].try_into().ok()?);
    Some((tdev, ppid))
}

#[cfg(target_os = "macos")]
fn tty_width_from_dev(tdev: i32) -> Option<usize> {
    if tdev == -1 || tdev == 0 {
        return None;
    }
    // macOS dev_t: major = (tdev >> 24) & 0xff, minor = tdev & 0xffffff
    let major = ((tdev >> 24) & 0xff) as u32;
    let minor = (tdev & 0xffffff) as u32;
    // macOS pseudo-terminals have major 16
    if major != 16 {
        return None;
    }
    let path = format!("/dev/ttys{minor:03}");
    let f = std::fs::File::open(&path).ok()?;
    terminal_size::terminal_size_of(&f).map(|(w, _)| w.0 as usize)
}

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

    let mut buf = String::new();
    let mut input: input::Input = if cli.demo {
        input::demo()
    } else {
        if std::io::stdin().read_to_string(&mut buf).is_err() {
            return;
        }
        match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(_) => return,
        }
    };

    if input.git_branch.is_none()
        && let Some(ref cwd) = input.cwd
    {
        let gi = git::info(cwd);
        input.git_branch = gi.branch;
        input.git_commit = gi.sha;
        input.git_tag = gi.tag;
    }

    if config.stats.enabled && !cli.demo {
        match serde_json::from_str(&buf) {
            Ok(serde_json::Value::Object(raw)) => stats::append_record(&raw),
            _ => config::debug(|| "stats: could not re-parse input as JSON object".into()),
        }
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

    let parent_width = probe_parent_tty_width();
    let ctx_pct = input.ctx_pct().unwrap_or(0.0);
    let margin = if ctx_pct >= 80.0 { config.width_margin } else { 0 };

    let max_width: Option<usize> = match cli.width {
        Some(0) => None,
        Some(w) => Some(w),
        None => parent_width
            .map(|w| w.saturating_sub(margin)),
    };

    config::debug(|| format!("max_width: {max_width:?} (cli={:?}, parent_tty={parent_width:?}, ctx={ctx_pct:.0}%, margin={margin})",
        cli.width,
    ));

    let out = render::render_all(&bar_lines, &input, icon_mode, &config, &agg_stats, max_width);

    print!("{out}");
}
