use crate::config::{Element, IconMode};
use crate::format::{format_duration, format_tokens, shorten_path};
use crate::input::Input;

const BRAILLE: [char; 9] = [
    '\u{2800}', '\u{2840}', '\u{2844}', '\u{2846}', '\u{2847}',
    '\u{28C7}', '\u{28E7}', '\u{28F7}', '\u{28FF}',
];

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const BG_RED: &str = "\x1b[41m";
const WHITE: &str = "\x1b[97m";
const DIM: &str = "\x1b[2m";
const RST: &str = "\x1b[0m";

fn gauge(pct: f64, width: usize, color: &str) -> String {
    let fill = pct / 100.0 * width as f64;
    (0..width)
        .map(|i| {
            let idx = ((fill - i as f64).clamp(0.0, 1.0) * 8.0).round() as usize;
            let c = if idx > 0 { color } else { DIM };
            format!("{c}{}{RST}", BRAILLE[idx])
        })
        .collect()
}

fn pct_color(pct: f64) -> &'static str {
    match () {
        _ if pct >= 80.0 => RED,
        _ if pct >= 50.0 => YELLOW,
        _ => GREEN,
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

pub fn render(elem: Element, input: &Input, mode: IconMode) -> Option<String> {
    let i = icon(elem, mode);
    match elem {
        Element::Model => {
            let name = input.model.as_ref()?.display_name.as_ref()?;
            Some(format!("{i}{CYAN}{name}{RST}"))
        }
        Element::Version => {
            let v = input.version.as_ref()?;
            Some(format!("{i}{DIM}v{v}{RST}"))
        }
        Element::Gauge => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            let g = gauge(pct, 10, pct_color(pct));
            let mut out = format!("{i}{g}");
            if input.exceeds_200k_tokens.unwrap_or(false) {
                out.push_str(&format!(" {BG_RED}{WHITE} CTX EXCEEDED {RST}"));
            }
            Some(out)
        }
        Element::Context => {
            let pct = input.context_window.as_ref()?.used_percentage?;
            Some(format!("{i}{}{pct:.0}%{RST}", pct_color(pct)))
        }
        Element::Tokens => {
            let cw = input.context_window.as_ref()?;
            let inp = format_tokens(cw.total_input_tokens?);
            let out = format_tokens(cw.total_output_tokens?);
            Some(format!("{i}{DIM}{inp}/{out}{RST}"))
        }
        Element::Cache => {
            let u = input.context_window.as_ref()?.current_usage.as_ref()?;
            let r = u.cache_read_input_tokens.unwrap_or(0);
            let w = u.cache_creation_input_tokens.unwrap_or(0);
            if r == 0 && w == 0 { return None; }
            Some(format!("{i}{DIM}r:{} w:{}{RST}", format_tokens(r), format_tokens(w)))
        }
        Element::Cost => {
            let c = input.cost.as_ref()?.total_cost_usd?;
            Some(format!("{i}{DIM}${c:.2}{RST}"))
        }
        Element::Lines => {
            let cost = input.cost.as_ref()?;
            let a = cost.total_lines_added.unwrap_or(0);
            let d = cost.total_lines_removed.unwrap_or(0);
            if a == 0 && d == 0 { return None; }
            Some(format!("{i}{GREEN}+{a}{RST}/{RED}-{d}{RST}"))
        }
        Element::Duration => {
            let ms = input.cost.as_ref()?.total_api_duration_ms?;
            Some(format!("{i}{DIM}{}{RST}", format_duration(ms)))
        }
        Element::Cwd => {
            let p = input.cwd.as_ref()?;
            Some(format!("{i}{DIM}{}{RST}", shorten_path(p)))
        }
        Element::ProjectDir => {
            let p = input.workspace.as_ref()?.project_dir.as_ref()?;
            Some(format!("{i}{DIM}{}{RST}", shorten_path(p)))
        }
        Element::OutputStyle => {
            let name = input.output_style.as_ref()?.name.as_ref()?;
            if name == "default" { return None; }
            Some(format!("{i}{DIM}[{name}]{RST}"))
        }
    }
}
