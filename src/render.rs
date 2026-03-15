use crate::config::{
    Element, IconMode, Icons, ALERT_ICONS, CACHE_ICONS, CONTEXT_ICONS, COST_ICONS, CWD_ICONS,
    DURATION_ICONS, LINES_ICONS, MODEL_ICONS, PROJECT_ICONS, STYLE_ICONS, TOKENS_ICONS,
    VERSION_ICONS,
};
use crate::format::{format_duration, format_tokens, shorten_path};
use crate::input::Input;
use crate::style::{apply_style, parse_style};
use crate::toml_config::BarConfig;

const BRAILLE: [char; 9] = [
    '\u{2800}', '\u{2840}', '\u{2844}', '\u{2846}', '\u{2847}', '\u{28C7}', '\u{28E7}', '\u{28F7}',
    '\u{28FF}',
];
const SHADE: [char; 5] = [' ', '░', '▒', '▓', '█'];
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RST: &str = "\x1b[0m";

const BG_RED: &str = "\x1b[41m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const WHITE: &str = "\x1b[97m";
const BLACK: &str = "\x1b[30m";

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BarStyle {
    Braille,
    Block,
    Shade,
    Ascii,
}

pub fn parse_bar_style(s: &str) -> BarStyle {
    match s {
        "block" => BarStyle::Block,
        "shade" => BarStyle::Shade,
        "ascii" => BarStyle::Ascii,
        _ => BarStyle::Braille,
    }
}

fn braille_bar(pct: f64, width: usize, color: &str) -> String {
    let fill = pct / 100.0 * width as f64;
    let mut out = String::with_capacity(width * 16);
    for i in 0..width {
        let idx = ((fill - i as f64).clamp(0.0, 1.0) * 8.0).round() as usize;
        let c = if idx > 0 { color } else { DIM };
        out.push_str(c);
        out.push(BRAILLE[idx]);
        out.push_str(RST);
    }
    out
}

fn block_bar(pct: f64, width: usize, color: &str) -> String {
    let filled = ((pct / 100.0 * width as f64).round() as usize).min(width);
    let mut out = String::with_capacity(width * 16);
    for _ in 0..filled {
        out.push_str(color);
        out.push('▰');
        out.push_str(RST);
    }
    for _ in filled..width {
        out.push_str(DIM);
        out.push('▱');
        out.push_str(RST);
    }
    out
}

fn shade_bar(pct: f64, width: usize, color: &str) -> String {
    let fill = pct / 100.0 * width as f64;
    let mut out = String::with_capacity(width * 16);
    for i in 0..width {
        let level = ((fill - i as f64).clamp(0.0, 1.0) * 4.0).round() as usize;
        let c = if level > 0 { color } else { DIM };
        out.push_str(c);
        out.push(SHADE[level]);
        out.push_str(RST);
    }
    out
}

fn ascii_bar(pct: f64, width: usize, color: &str) -> String {
    let filled = ((pct / 100.0 * width as f64).round() as usize).min(width);
    let mut out = String::with_capacity(width + 16);
    out.push('[');
    out.push_str(color);
    for _ in 0..filled {
        out.push('#');
    }
    out.push_str(RST);
    out.push_str(DIM);
    for _ in filled..width {
        out.push('-');
    }
    out.push_str(RST);
    out.push(']');
    out
}

pub fn render_bar(pct: f64, style: BarStyle, width: usize, color: &str) -> String {
    match style {
        BarStyle::Braille => braille_bar(pct, width, color),
        BarStyle::Block => block_bar(pct, width, color),
        BarStyle::Shade => shade_bar(pct, width, color),
        BarStyle::Ascii => ascii_bar(pct, width, color),
    }
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

fn icon<'a>(custom: &'a str, mode: IconMode, icons: &'static Icons) -> &'a str {
    if !custom.is_empty() && custom != icons.oct {
        custom
    } else {
        match mode {
            IconMode::None => icons.none,
            IconMode::Octicons => icons.oct,
            IconMode::FontAwesome => icons.fa,
        }
    }
}

fn render_element(
    symbol: &str,
    style: &str,
    mode: IconMode,
    icons: &'static Icons,
    value: Option<String>,
) -> Option<String> {
    let v = value?;
    let i = icon(symbol, mode, icons);
    let ansi = parse_style(style);
    Some(format!("{i}{ansi}{v}{RST}"))
}

fn render_model(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let name = input.model.as_ref()?.display_name.as_ref()?;
    render_element(&config.model.symbol, &config.model.style, mode,
        &MODEL_ICONS, Some(name.to_string()))
}

fn render_version(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let v = input.version.as_ref()?;
    render_element(&config.version.symbol, &config.version.style, mode,
        &VERSION_ICONS, Some(format!("v{v}")))
}

fn render_context(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.context;
    let pct = input.context_window.as_ref()?.used_percentage?;
    if !cfg.show_bar && !cfg.show_pct { return None; }

    let i = icon(&cfg.symbol, mode, &CONTEXT_ICONS);
    let color = pct_color(pct);
    let bar_style = parse_bar_style(&cfg.bar_style);

    let mut out = String::from(i);

    if cfg.show_bar {
        out.push_str(&render_bar(pct, bar_style, cfg.width, color));
    }
    if cfg.show_bar && cfg.show_pct {
        out.push(' ');
    }
    if cfg.show_pct {
        out.push_str(&format!("{color}{pct:.0}%{RST}"));
    }

    if !cfg.style.is_empty() {
        Some(apply_style(&out, &cfg.style))
    } else {
        Some(out)
    }
}

fn render_tokens(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cw = input.context_window.as_ref()?;
    let inp = format_tokens(cw.total_input_tokens?);
    let out = format_tokens(cw.total_output_tokens?);
    render_element(&config.tokens.symbol, &config.tokens.style, mode,
        &TOKENS_ICONS, Some(format!("{inp}/{out}")))
}

fn render_cache(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let u = input.context_window.as_ref()?.current_usage.as_ref()?;
    let r = u.cache_read_input_tokens.unwrap_or(0);
    let w = u.cache_creation_input_tokens.unwrap_or(0);
    if r == 0 && w == 0 { return None; }
    render_element(&config.cache.symbol, &config.cache.style, mode,
        &CACHE_ICONS,
        Some(format!("r:{} w:{}", format_tokens(r), format_tokens(w))))
}

fn render_cost(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let c = input.cost.as_ref()?.total_cost_usd?;
    render_element(&config.cost.symbol, &config.cost.style, mode,
        &COST_ICONS, Some(format!("${c:.2}")))
}

fn render_lines(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let cfg = &config.lines;
    let i = icon(&cfg.symbol, mode, &LINES_ICONS);
    let cost = input.cost.as_ref()?;
    let a = cost.total_lines_added.unwrap_or(0);
    let d = cost.total_lines_removed.unwrap_or(0);
    if a == 0 && d == 0 { return None; }
    let content = format!("{i}{GREEN}+{a}{RST}/{RED}-{d}{RST}");
    Some(apply_style(&content, &cfg.style))
}

fn render_duration(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let ms = input.cost.as_ref()?.total_api_duration_ms?;
    render_element(&config.duration.symbol, &config.duration.style, mode,
        &DURATION_ICONS, Some(format_duration(ms)))
}

fn render_cwd(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let p = input.cwd.as_ref()?;
    render_element(&config.cwd.symbol, &config.cwd.style, mode,
        &CWD_ICONS, Some(shorten_path(p)))
}

fn render_project(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let p = input.workspace.as_ref()?.project_dir.as_ref()?;
    render_element(&config.project.symbol, &config.project.style, mode,
        &PROJECT_ICONS, Some(shorten_path(p)))
}

fn render_output_style(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let name = input.output_style.as_ref()?.name.as_ref()?;
    if name == "default" { return None; }
    render_element(&config.style.symbol, &config.style.style, mode,
        &STYLE_ICONS, Some(format!("[{name}]")))
}

fn severity_style(severity: &str) -> (&'static str, &'static str) {
    match severity {
        "warn" => (BG_YELLOW, BLACK),
        "info" => (BG_BLUE, WHITE),
        _ => (BG_RED, WHITE),
    }
}

fn render_alert(input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let mut badges = Vec::new();

    for rule in &config.alerts {
        let fires = match rule.trigger.as_str() {
            "ctx_exceeded" => input.exceeds_200k_tokens.unwrap_or(false),
            "ctx_high" => {
                if let Some(threshold) = rule.threshold {
                    input.context_window.as_ref()
                        .and_then(|cw| cw.used_percentage)
                        .is_some_and(|pct| pct >= threshold)
                } else {
                    false
                }
            }
            _ => false,
        };

        if fires {
            let (bg, fg) = severity_style(&rule.severity);
            let i = icon("", mode, &ALERT_ICONS);
            badges.push(format!("{i}{bg}{fg} {} {RST}", rule.label));
        }
    }

    if badges.is_empty() {
        None
    } else {
        Some(badges.join(" "))
    }
}

pub fn render(elem: Element, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    match elem {
        Element::Model => render_model(input, mode, config),
        Element::Version => render_version(input, mode, config),
        Element::Context => render_context(input, mode, config),
        Element::Tokens => render_tokens(input, mode, config),
        Element::Cache => render_cache(input, mode, config),
        Element::Cost => render_cost(input, mode, config),
        Element::Lines => render_lines(input, mode, config),
        Element::Duration => render_duration(input, mode, config),
        Element::Cwd => render_cwd(input, mode, config),
        Element::ProjectDir => render_project(input, mode, config),
        Element::OutputStyle => render_output_style(input, mode, config),
        Element::Alert => render_alert(input, mode, config),
    }
}
