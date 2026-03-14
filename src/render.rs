use crate::config::{Element, IconMode};
use crate::format::{format_duration, format_tokens, shorten_path};
use crate::input::Input;
use crate::style::apply_style;
use crate::toml_config::BarConfig;

const BRAILLE: [char; 9] = [
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
const RST: &str = "\x1b[0m";

fn gauge_bar(pct: f64, width: usize, color: &str) -> String {
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
    if pct >= 80.0 {
        RED
    } else if pct >= 50.0 {
        YELLOW
    } else {
        GREEN
    }
}

fn icon<'a>(
    custom: &'a str,
    mode: IconMode,
    none: &'static str,
    oct: &'static str,
    fa: &'static str,
) -> &'a str {
    if !custom.is_empty() && custom != oct {
        custom
    } else {
        match mode {
            IconMode::None => none,
            IconMode::Octicons => oct,
            IconMode::FontAwesome => fa,
        }
    }
}

fn styled(content: String, cfg_style: &str, default_style: &str) -> String {
    if cfg_style != default_style {
        apply_style(&content, cfg_style)
    } else {
        content
    }
}

fn render_model(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.model;
    let i = icon(&cfg.symbol, mode, "", "\u{f4be} ", "\u{ee0d} ");
    let name = input.model.as_ref()?.display_name.as_ref()?;
    Some(styled(format!("{i}{CYAN}{name}{RST}"), &cfg.style, "cyan"))
}

fn render_version(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.version;
    let i = icon(&cfg.symbol, mode, "", "\u{f412} ", "\u{f02b} ");
    let v = input.version.as_ref()?;
    Some(styled(format!("{i}{DIM}v{v}{RST}"), &cfg.style, "dim"))
}

fn render_gauge(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.gauge;
    let i = icon(&cfg.symbol, mode, "", "\u{f4ed} ", "\u{ef0d} ");
    let pct = input.context_window.as_ref()?.used_percentage?;
    let g = gauge_bar(pct, 10, pct_color(pct));
    let mut out = format!("{i}{g}");
    if input.exceeds_200k_tokens.unwrap_or(false) {
        out.push_str(&format!(" {BG_RED}{WHITE} CTX EXCEEDED {RST}"));
    }
    Some(out)
}

fn render_context(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.context;
    let i = icon(&cfg.symbol, mode, "", "\u{f463} ", "\u{eeb2} ");
    let pct = input.context_window.as_ref()?.used_percentage?;
    Some(format!("{i}{}{pct:.0}%{RST}", pct_color(pct)))
}

fn render_tokens(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.tokens;
    let i = icon(&cfg.symbol, mode, "", "\u{f4df} ", "\u{f292} ");
    let cw = input.context_window.as_ref()?;
    let inp = format_tokens(cw.total_input_tokens?);
    let out = format_tokens(cw.total_output_tokens?);
    Some(styled(format!("{i}{DIM}{inp}/{out}{RST}"), &cfg.style, "dim"))
}

fn render_cache(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.cache;
    let i = icon(&cfg.symbol, mode, "", "\u{f49b} ", "\u{f1c0} ");
    let u = input.context_window.as_ref()?.current_usage.as_ref()?;
    let r = u.cache_read_input_tokens.unwrap_or(0);
    let w = u.cache_creation_input_tokens.unwrap_or(0);
    if r == 0 && w == 0 { return None; }
    Some(styled(
        format!("{i}{DIM}r:{} w:{}{RST}", format_tokens(r), format_tokens(w)),
        &cfg.style,
        "dim",
    ))
}

fn render_cost(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.cost;
    let i = icon(&cfg.symbol, mode, "", "\u{f439} ", "\u{f09d} ");
    let c = input.cost.as_ref()?.total_cost_usd?;
    Some(styled(format!("{i}{DIM}${c:.2}{RST}"), &cfg.style, "dim"))
}

fn render_lines(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.lines;
    let i = icon(&cfg.symbol, mode, "", "\u{f4d2} ", "\u{f05f} ");
    let cost = input.cost.as_ref()?;
    let a = cost.total_lines_added.unwrap_or(0);
    let d = cost.total_lines_removed.unwrap_or(0);
    if a == 0 && d == 0 { return None; }
    Some(format!("{i}{GREEN}+{a}{RST}/{RED}-{d}{RST}"))
}

fn render_duration(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.duration;
    let i = icon(&cfg.symbol, mode, "api:", "\u{f4e3} ", "\u{f254} ");
    let ms = input.cost.as_ref()?.total_api_duration_ms?;
    Some(styled(format!("{i}{DIM}{}{RST}", format_duration(ms)), &cfg.style, "dim"))
}

fn render_cwd(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.cwd;
    let i = icon(&cfg.symbol, mode, "cwd:", "\u{f413} ", "\u{f114} ");
    let p = input.cwd.as_ref()?;
    Some(styled(format!("{i}{DIM}{}{RST}", shorten_path(p)), &cfg.style, "dim"))
}

fn render_project(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.project;
    let i = icon(&cfg.symbol, mode, "proj:", "\u{f46d} ", "\u{f015} ");
    let p = input.workspace.as_ref()?.project_dir.as_ref()?;
    Some(styled(format!("{i}{DIM}{}{RST}", shorten_path(p)), &cfg.style, "dim"))
}

fn render_output_style(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.style;
    let i = icon(&cfg.symbol, mode, "", "\u{f48f} ", "\u{f1fc} ");
    let name = input.output_style.as_ref()?.name.as_ref()?;
    if name == "default" { return None; }
    Some(styled(format!("{i}{DIM}[{name}]{RST}"), &cfg.style, "dim"))
}

pub fn render(elem: Element, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    match elem {
        Element::Model => render_model(input, mode, config),
        Element::Version => render_version(input, mode, config),
        Element::Gauge => render_gauge(input, mode, config),
        Element::Context => render_context(input, mode, config),
        Element::Tokens => render_tokens(input, mode, config),
        Element::Cache => render_cache(input, mode, config),
        Element::Cost => render_cost(input, mode, config),
        Element::Lines => render_lines(input, mode, config),
        Element::Duration => render_duration(input, mode, config),
        Element::Cwd => render_cwd(input, mode, config),
        Element::ProjectDir => render_project(input, mode, config),
        Element::OutputStyle => render_output_style(input, mode, config),
    }
}
