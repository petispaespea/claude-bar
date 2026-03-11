use crate::Input;
use crate::config::{Element, IconMode};
use crate::format::{format_duration, format_tokens, shorten_path};

const BRAILLE_LEVELS: [char; 9] = [
    '\u{2800}', '\u{2840}', '\u{2844}', '\u{2846}', '\u{2847}', '\u{28C7}', '\u{28E7}', '\u{28F7}',
    '\u{28FF}',
];

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const BG_RED: &str = "\x1b[41m";
const WHITE: &str = "\x1b[97m";
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

fn icon(elem: Element, mode: IconMode) -> &'static str {
    match mode {
        IconMode::None => match elem {
            Element::Duration => "api:",
            Element::Cwd => "cwd:",
            Element::ProjectDir => "proj:",
            _ => "",
        },
        IconMode::Octicons => match elem {
            Element::Model => "\u{f4be} ",
            Element::Version => "\u{f412} ",
            Element::Gauge => "\u{f4ed} ",
            Element::Context => "\u{f463} ",
            Element::Tokens => "\u{f4df} ",
            Element::Cache => "\u{f49b} ",
            Element::Cost => "\u{f439} ",
            Element::Lines => "\u{f4d2} ",
            Element::Duration => "\u{f4e3} ",
            Element::Cwd => "\u{f413} ",
            Element::ProjectDir => "\u{f46d} ",
            Element::OutputStyle => "\u{f48f} ",
        },
        IconMode::FontAwesome => match elem {
            Element::Model => "\u{ee0d} ",
            Element::Version => "\u{f02b} ",
            Element::Gauge => "\u{ef0d} ",
            Element::Context => "\u{eeb2} ",
            Element::Tokens => "\u{f292} ",
            Element::Cache => "\u{f1c0} ",
            Element::Cost => "\u{f09d} ",
            Element::Lines => "\u{f05f} ",
            Element::Duration => "\u{f254} ",
            Element::Cwd => "\u{f114} ",
            Element::ProjectDir => "\u{f015} ",
            Element::OutputStyle => "\u{f1fc} ",
        },
    }
}

pub fn render_element(elem: Element, input: &Input, icon_mode: IconMode) -> Option<String> {
    let prefix = icon(elem, icon_mode);
    match elem {
        Element::Model => {
            let name = input.model.as_ref()?.display_name.as_ref()?;
            Some(format!("{prefix}{CYAN}{name}{RESET}"))
        }
        Element::Version => {
            let v = input.version.as_ref()?;
            Some(format!("{prefix}{DIM}v{v}{RESET}"))
        }
        Element::Gauge => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            let color = pct_color(pct);
            let gauge = braille_gauge(pct, 10, color);
            let mut result = format!("{prefix}{gauge}");
            if input.exceeds_200k_tokens.unwrap_or(false) {
                result.push_str(&format!(" {BG_RED}{WHITE} CTX EXCEEDED {RESET}"));
            }
            Some(result)
        }
        Element::Context => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            let color = pct_color(pct);
            Some(format!("{prefix}{color}{pct:.0}%{RESET}"))
        }
        Element::Tokens => {
            let cw = input.context_window.as_ref()?;
            let inp = cw.total_input_tokens?;
            let out = cw.total_output_tokens?;
            Some(format!(
                "{prefix}{DIM}{}{RESET}{DIM}/{RESET}{DIM}{}{RESET}",
                format_tokens(inp),
                format_tokens(out)
            ))
        }
        Element::Cache => {
            let usage = input.context_window.as_ref()?.current_usage.as_ref()?;
            let read = usage.cache_read_input_tokens.unwrap_or(0);
            let write = usage.cache_creation_input_tokens.unwrap_or(0);
            if read == 0 && write == 0 {
                return None;
            }
            Some(format!(
                "{prefix}{DIM}r:{} w:{}{RESET}",
                format_tokens(read),
                format_tokens(write)
            ))
        }
        Element::Cost => {
            let c = input.cost.as_ref()?.total_cost_usd?;
            Some(format!("{prefix}{DIM}${c:.2}{RESET}"))
        }
        Element::Lines => {
            let cost = input.cost.as_ref()?;
            let added = cost.total_lines_added.unwrap_or(0);
            let removed = cost.total_lines_removed.unwrap_or(0);
            if added == 0 && removed == 0 {
                return None;
            }
            Some(format!(
                "{prefix}{GREEN}+{added}{RESET}/{RED}-{removed}{RESET}"
            ))
        }
        Element::Duration => {
            let cost = input.cost.as_ref()?;
            let api = cost.total_api_duration_ms.map(format_duration)?;
            Some(format!("{prefix}{DIM}{api}{RESET}"))
        }
        Element::Cwd => {
            let p = input.cwd.as_ref()?;
            Some(format!("{prefix}{DIM}{}{RESET}", shorten_path(p)))
        }
        Element::ProjectDir => {
            let p = input.workspace.as_ref()?.project_dir.as_ref()?;
            Some(format!("{prefix}{DIM}{}{RESET}", shorten_path(p)))
        }
        Element::OutputStyle => {
            let name = input.output_style.as_ref()?.name.as_ref()?;
            if name == "default" {
                return None;
            }
            Some(format!("{prefix}{DIM}[{name}]{RESET}"))
        }
    }
}
