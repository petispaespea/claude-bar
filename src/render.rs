use crate::config::Element;
use crate::format::{format_duration, shorten_path};
use crate::Input;

const BRAILLE_LEVELS: [char; 9] = [
    '\u{2800}', '\u{2840}', '\u{2844}', '\u{2846}',
    '\u{2847}', '\u{28C7}', '\u{28E7}', '\u{28F7}', '\u{28FF}',
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

fn icon(elem: Element) -> &'static str {
    match elem {
        Element::Model => "\u{e99a} ",
        Element::Version => "\u{f412} ",
        Element::Gauge => "\u{f0e70} ",
        Element::Context => "\u{f0201} ",
        Element::Cost => "\u{f155} ",
        Element::Lines => "\u{f0dca} ",
        Element::Duration => "\u{f253} ",
        Element::Cwd => "\u{f115} ",
        Element::ProjectDir => "\u{f07c} ",
        Element::OutputStyle => "\u{f1fc} ",
    }
}

pub fn render_element(elem: Element, input: &Input, show_icons: bool) -> Option<String> {
    let prefix = if show_icons { icon(elem) } else { "" };
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
            Some(format!("{prefix}{GREEN}+{added}{RESET}/{RED}-{removed}{RESET}"))
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
