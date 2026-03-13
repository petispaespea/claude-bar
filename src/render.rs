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

pub trait Module {
    #[allow(dead_code)]
    fn element(&self) -> Element;
    fn default_icon(&self, mode: IconMode) -> &'static str;
    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String>;
}

pub struct ModelModule;

impl Module for ModelModule {
    fn element(&self) -> Element {
        Element::Model
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f4be} ",
            IconMode::FontAwesome => "\u{ee0d} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.model;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let name = input.model.as_ref()?.display_name.as_ref()?;
        let content = format!("{i}{CYAN}{name}{RST}");
        if cfg.style != "cyan" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct VersionModule;

impl Module for VersionModule {
    fn element(&self) -> Element {
        Element::Version
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f412} ",
            IconMode::FontAwesome => "\u{f02b} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.version;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let v = input.version.as_ref()?;
        let content = format!("{i}{DIM}v{v}{RST}");
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct GaugeModule;

impl Module for GaugeModule {
    fn element(&self) -> Element {
        Element::Gauge
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f4ed} ",
            IconMode::FontAwesome => "\u{ef0d} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.gauge;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let pct = input.context_window.as_ref()?.used_percentage?;
        let g = gauge(pct, 10, pct_color(pct));
        let mut out = format!("{i}{g}");
        if input.exceeds_200k_tokens.unwrap_or(false) {
            out.push_str(&format!(" {BG_RED}{WHITE} CTX EXCEEDED {RST}"));
        }
        Some(out)
    }
}

pub struct ContextModule;

impl Module for ContextModule {
    fn element(&self) -> Element {
        Element::Context
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f463} ",
            IconMode::FontAwesome => "\u{eeb2} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.context;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let pct = input.context_window.as_ref()?.used_percentage?;
        Some(format!("{i}{}{pct:.0}%{RST}", pct_color(pct)))
    }
}

pub struct TokensModule;

impl Module for TokensModule {
    fn element(&self) -> Element {
        Element::Tokens
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f4df} ",
            IconMode::FontAwesome => "\u{f292} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.tokens;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let cw = input.context_window.as_ref()?;
        let inp = format_tokens(cw.total_input_tokens?);
        let out = format_tokens(cw.total_output_tokens?);
        let content = format!("{i}{DIM}{inp}/{out}{RST}");
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct CacheModule;

impl Module for CacheModule {
    fn element(&self) -> Element {
        Element::Cache
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f49b} ",
            IconMode::FontAwesome => "\u{f1c0} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.cache;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let u = input.context_window.as_ref()?.current_usage.as_ref()?;
        let r = u.cache_read_input_tokens.unwrap_or(0);
        let w = u.cache_creation_input_tokens.unwrap_or(0);
        if r == 0 && w == 0 {
            return None;
        }
        let content = format!("{i}{DIM}r:{} w:{}{RST}", format_tokens(r), format_tokens(w));
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct CostModule;

impl Module for CostModule {
    fn element(&self) -> Element {
        Element::Cost
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f439} ",
            IconMode::FontAwesome => "\u{f09d} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.cost;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let c = input.cost.as_ref()?.total_cost_usd?;
        let content = format!("{i}{DIM}${c:.2}{RST}");
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct LinesModule;

impl Module for LinesModule {
    fn element(&self) -> Element {
        Element::Lines
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f4d2} ",
            IconMode::FontAwesome => "\u{f05f} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.lines;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let cost = input.cost.as_ref()?;
        let a = cost.total_lines_added.unwrap_or(0);
        let d = cost.total_lines_removed.unwrap_or(0);
        if a == 0 && d == 0 {
            return None;
        }
        Some(format!("{i}{GREEN}+{a}{RST}/{RED}-{d}{RST}"))
    }
}

pub struct DurationModule;

impl Module for DurationModule {
    fn element(&self) -> Element {
        Element::Duration
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "api:",
            IconMode::Octicons => "\u{f4e3} ",
            IconMode::FontAwesome => "\u{f254} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.duration;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let ms = input.cost.as_ref()?.total_api_duration_ms?;
        let content = format!("{i}{DIM}{}{RST}", format_duration(ms));
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct CwdModule;

impl Module for CwdModule {
    fn element(&self) -> Element {
        Element::Cwd
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "cwd:",
            IconMode::Octicons => "\u{f413} ",
            IconMode::FontAwesome => "\u{f114} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.cwd;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let p = input.cwd.as_ref()?;
        let content = format!("{i}{DIM}{}{RST}", shorten_path(p));
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct ProjectDirModule;

impl Module for ProjectDirModule {
    fn element(&self) -> Element {
        Element::ProjectDir
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "proj:",
            IconMode::Octicons => "\u{f46d} ",
            IconMode::FontAwesome => "\u{f015} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.project;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let p = input.workspace.as_ref()?.project_dir.as_ref()?;
        let content = format!("{i}{DIM}{}{RST}", shorten_path(p));
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub struct OutputStyleModule;

impl Module for OutputStyleModule {
    fn element(&self) -> Element {
        Element::OutputStyle
    }

    fn default_icon(&self, mode: IconMode) -> &'static str {
        match mode {
            IconMode::None => "",
            IconMode::Octicons => "\u{f48f} ",
            IconMode::FontAwesome => "\u{f1fc} ",
        }
    }

    fn render(&self, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
        let cfg = &config.style;
        if cfg.disabled {
            return None;
        }
        let i = if !cfg.symbol.is_empty()
            && cfg.symbol.as_str() != self.default_icon(IconMode::Octicons)
        {
            cfg.symbol.as_str()
        } else {
            self.default_icon(mode)
        };
        let name = input.output_style.as_ref()?.name.as_ref()?;
        if name == "default" {
            return None;
        }
        let content = format!("{i}{DIM}[{name}]{RST}");
        if cfg.style != "dim" {
            Some(apply_style(&content, &cfg.style))
        } else {
            Some(content)
        }
    }
}

pub fn render(elem: Element, input: &Input, mode: IconMode, config: &BarConfig) -> Option<String> {
    let module: Box<dyn Module> = match elem {
        Element::Model => Box::new(ModelModule),
        Element::Version => Box::new(VersionModule),
        Element::Gauge => Box::new(GaugeModule),
        Element::Context => Box::new(ContextModule),
        Element::Tokens => Box::new(TokensModule),
        Element::Cache => Box::new(CacheModule),
        Element::Cost => Box::new(CostModule),
        Element::Lines => Box::new(LinesModule),
        Element::Duration => Box::new(DurationModule),
        Element::Cwd => Box::new(CwdModule),
        Element::ProjectDir => Box::new(ProjectDirModule),
        Element::OutputStyle => Box::new(OutputStyleModule),
    };
    module.render(input, mode, config)
}
