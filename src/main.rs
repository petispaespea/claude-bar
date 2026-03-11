use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
struct Input {
    model: Option<Model>,
    context_window: Option<ContextWindow>,
    cost: Option<Cost>,
    cwd: Option<String>,
}

#[derive(Deserialize)]
struct Model {
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct ContextWindow {
    used_percentage: Option<f64>,
}

#[derive(Deserialize)]
struct Cost {
    total_cost_usd: Option<f64>,
    total_lines_added: Option<i64>,
    total_lines_removed: Option<i64>,
}

const BRAILLE_LEVELS: [char; 9] = [
    '\u{2800}', // ⠀ empty
    '\u{2840}', // ⡀
    '\u{2844}', // ⡄
    '\u{2846}', // ⡆
    '\u{2847}', // ⡇
    '\u{28C7}', // ⣇
    '\u{28E7}', // ⣧
    '\u{28F7}', // ⣷
    '\u{28FF}', // ⣿ full
];

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const MAGENTA: &str = "\x1b[35m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn braille_gauge(percentage: f64, width: usize, fill_color: &str) -> String {
    let fill = percentage / 100.0 * width as f64;
    let mut gauge = String::new();

    for i in 0..width {
        let level = (fill - i as f64).clamp(0.0, 1.0);
        let idx = (level * 8.0).round() as usize;
        if idx > 0 {
            gauge.push_str(fill_color);
        } else {
            gauge.push_str(DIM);
        }
        gauge.push(BRAILLE_LEVELS[idx]);
        gauge.push_str(RESET);
    }

    gauge
}

fn pct_color(pct: f64) -> &'static str {
    if pct >= 80.0 {
        RED
    } else if pct >= 50.0 {
        YELLOW
    } else {
        GREEN
    }
}

fn shorten_path(path: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let shortened = if !home.is_empty() && path.starts_with(&home) {
        format!("~{}", &path[home.len()..])
    } else {
        path.to_string()
    };

    let parts: Vec<&str> = shortened.split('/').collect();
    if parts.len() <= 3 {
        return shortened;
    }
    let last_two = &parts[parts.len() - 2..];
    format!("…/{}", last_two.join("/"))
}

fn main() {
    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        return;
    }

    let input: Input = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(_) => return,
    };

    let model = input
        .model
        .and_then(|m| m.display_name)
        .unwrap_or_else(|| "?".into());

    let ctx_part = match input.context_window.and_then(|cw| cw.used_percentage) {
        Some(pct) => {
            let color = pct_color(pct);
            let gauge = braille_gauge(pct, 10, color);
            format!("{color}{gauge}{RESET}  {pct:.0}%")
        }
        None => "ctx: -".into(),
    };

    let cost_part = input
        .cost
        .as_ref()
        .and_then(|c| c.total_cost_usd)
        .map(|c| format!("  {DIM}${c:.2}{RESET}"))
        .unwrap_or_default();

    let lines_part = match &input.cost {
        Some(c) => {
            let added = c.total_lines_added.unwrap_or(0);
            let removed = c.total_lines_removed.unwrap_or(0);
            if added > 0 || removed > 0 {
                format!("  {GREEN}+{added}{RESET}/{RED}-{removed}{RESET}")
            } else {
                String::new()
            }
        }
        None => String::new(),
    };

    let cwd_part = input
        .cwd
        .map(|p| format!("  {DIM}{}{RESET}", shorten_path(&p)))
        .unwrap_or_default();

    print!("{CYAN}{model}{RESET}  {ctx_part}{cost_part}{lines_part}{cwd_part}");
}
