use std::io::IsTerminal;

use ansi_to_tui::IntoText;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::config::{
    self, BarItem, ALL_ELEMENT_NAMES, LINE_BREAK, STATS_ELEMENT_NAMES,
};
use crate::render;
use crate::stats::AggregateStats;
use crate::toml_config::{self, BarConfig};

const LAYOUT_NAMES: &[&str] = &["minimal", "compact", "default", "full", "loaded", "custom"];
const ICON_SET_NAMES: &[&str] = &["octicons", "fontawesome", "none"];
const BAR_STYLE_NAMES: &[&str] = &["braille", "block", "shade", "ascii", "progress"];

#[derive(Clone, Copy, PartialEq)]
enum Focus {
    BarList,
    Settings,
}

struct PickerEntry {
    item: BarItem,
    preview: Text<'static>,
    name: &'static str,
    description: &'static str,
}

struct PickerState {
    available: Vec<PickerEntry>,
    list_state: ListState,
}

struct App {
    items: Vec<BarItem>,
    loaded_items: Option<Vec<BarItem>>,
    list_state: ListState,

    focus: Focus,
    settings_field: usize,
    layout_idx: usize,
    stats_enabled: bool,
    budget_text: String,
    budget_editing: bool,
    icon_set_idx: usize,
    separator: String,
    separator_editing: bool,
    bar_style_idx: usize,

    picker: Option<PickerState>,
    preview_text: Text<'static>,
    bar_list_cache: Vec<ListItem<'static>>,
    confirm_quit: bool,
    should_quit: bool,
    saved: bool,
}

const SETTINGS_FIELD_COUNT: usize = 6;

fn cycle_index(current: usize, len: usize, up: bool) -> usize {
    if up {
        if current == 0 { len - 1 } else { current - 1 }
    } else {
        (current + 1) % len
    }
}

fn handle_text_edit(key: KeyEvent, text: &mut String, editing: &mut bool, filter: fn(char) -> bool) -> bool {
    if *editing {
        match key.code {
            KeyCode::Enter | KeyCode::Tab => {
                *editing = false;
                true
            }
            KeyCode::Esc => {
                *editing = false;
                false
            }
            KeyCode::Char(c) if filter(c) => {
                text.push(c);
                true
            }
            KeyCode::Backspace => {
                text.pop();
                true
            }
            _ => false,
        }
    } else {
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                *editing = true;
                false
            }
            _ => false,
        }
    }
}

fn find_matching_preset(items: &[BarItem]) -> Option<usize> {
    for (i, &name) in LAYOUT_NAMES.iter().enumerate() {
        if name == "custom" || name == "loaded" {
            continue;
        }
        if let Some(preset_lines) = config::preset_elements(name) {
            let preset_items = config::element_lines_to_bar_items(preset_lines);
            if preset_items == items {
                return Some(i);
            }
        }
    }
    None
}

fn active_indicator(active: bool) -> (&'static str, Style) {
    if active {
        ("▸", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    } else {
        (" ", Style::default())
    }
}

fn highlight_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::Green)
        .add_modifier(Modifier::BOLD)
}

fn section_border_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn field_label<'a>(indicator: &'a str, label: &'a str, style: Style) -> Span<'a> {
    Span::styled(format!("{indicator} {label}: "), style)
}

fn demo_stats() -> AggregateStats {
    AggregateStats {
        project_today_cost: 12.50,
        all_today_cost: 34.20,
        burn_rate: Some(8.15),
        spend_rate: Some(3.40),
        session_tok_per_dollar: Some(6952.0),
        cost_vs_avg: Some(1.2),
        ctx_trend: Some(5.0),
        daily_budget_pct: Some(34.2),
        avg_daily_cost: Some(28.50),
    }
}

fn render_item_preview(item: &BarItem, ctx: &DemoRenderCtx) -> Option<String> {
    match item {
        BarItem::Element(e) => {
            render::render(*e, &ctx.input, ctx.icon_mode, &ctx.config, &ctx.agg_stats)
        }
        BarItem::LineBreak => None,
    }
}

fn settings_field_count(stats_enabled: bool) -> usize {
    if stats_enabled { SETTINGS_FIELD_COUNT } else { SETTINGS_FIELD_COUNT - 1 }
}

struct DemoRenderCtx {
    input: crate::input::Input,
    config: BarConfig,
    icon_mode: crate::config::IconMode,
    agg_stats: Option<AggregateStats>,
}

impl App {
    fn new(existing: Option<BarConfig>) -> Self {
        let (items, loaded_items, stats_enabled, budget_text, icon_set_idx, separator, bar_style_idx) =
            if let Some(ref cfg) = existing {
                let items = config::bar_items_from_layout(&cfg.layout.elements);
                let loaded = items.clone();
                let stats = cfg.stats.enabled;
                let budget = format!("{:.0}", cfg.daily_budget.limit);
                let icon_idx = cfg
                    .icon_set
                    .as_deref()
                    .and_then(|s| ICON_SET_NAMES.iter().position(|&n| n == s))
                    .unwrap_or(0);
                let sep = cfg.separator.clone();
                let bar_idx = BAR_STYLE_NAMES
                    .iter()
                    .position(|&n| n == cfg.context.bar_style.as_str())
                    .unwrap_or(0);
                (items, Some(loaded), stats, budget, icon_idx, sep, bar_idx)
            } else {
                let default_lines = config::preset_elements("default").unwrap();
                let items = config::element_lines_to_bar_items(default_lines);
                (items, None, false, "100".into(), 0, " | ".into(), 0)
            };

        let layout_idx = find_matching_preset(&items)
            .unwrap_or(LAYOUT_NAMES.len() - 1);

        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }

        let mut app = App {
            items,
            loaded_items,
            list_state,
            focus: Focus::BarList,
            settings_field: 0,
            layout_idx,
            stats_enabled,
            budget_text,
            budget_editing: false,
            icon_set_idx,
            separator,
            separator_editing: false,
            bar_style_idx,
            picker: None,
            preview_text: Text::default(),
            bar_list_cache: Vec::new(),
            confirm_quit: false,
            should_quit: false,
            saved: false,
        };
        app.rebuild_preview();
        app
    }

    fn demo_render_ctx(&self) -> DemoRenderCtx {
        DemoRenderCtx {
            input: crate::input::demo(),
            config: self.build_config(),
            icon_mode: self.icon_mode(),
            agg_stats: if self.stats_enabled { Some(demo_stats()) } else { None },
        }
    }

    fn icon_mode(&self) -> config::IconMode {
        config::icon_mode_from_str(Some(ICON_SET_NAMES[self.icon_set_idx]))
    }

    fn build_config(&self) -> BarConfig {
        let layout_elements: Vec<String> = self.items.iter().map(|item| match item {
            BarItem::Element(e) => e.name().to_string(),
            BarItem::LineBreak => LINE_BREAK.to_string(),
        }).collect();

        let icon_set_str = ICON_SET_NAMES[self.icon_set_idx];
        let bar_style_str = BAR_STYLE_NAMES[self.bar_style_idx];

        BarConfig {
            separator: self.separator.clone(),
            icon_set: if self.icon_set_idx == 0 {
                None
            } else {
                Some(icon_set_str.to_string())
            },
            layout: toml_config::LayoutConfig { elements: layout_elements },
            context: toml_config::ContextConfig {
                bar_style: bar_style_str.to_string(),
                ..Default::default()
            },
            stats: toml_config::StatsConfig {
                enabled: self.stats_enabled,
                ..Default::default()
            },
            daily_budget: toml_config::DailyBudgetConfig {
                limit: self.budget_text.parse::<f64>().unwrap_or(100.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn rebuild_preview(&mut self) {
        let ctx = self.demo_render_ctx();
        let lines = config::bar_items_to_lines(&self.items);
        let out = render::render_all(&lines, &ctx.input, ctx.icon_mode, &ctx.config, &ctx.agg_stats);
        self.preview_text = out.into_text().unwrap_or_default();
        self.bar_list_cache = self.build_bar_list_items(&ctx);
    }

    fn sync_layout_from_items(&mut self) {
        if let Some(idx) = find_matching_preset(&self.items) {
            self.layout_idx = idx;
        } else {
            self.layout_idx = LAYOUT_NAMES.len() - 1; // custom
        }
    }

    fn apply_preset(&mut self) {
        let name = LAYOUT_NAMES[self.layout_idx];
        match name {
            "custom" => return,
            "loaded" => {
                if let Some(ref loaded) = self.loaded_items {
                    self.items = loaded.clone();
                }
            }
            _ => {
                if let Some(preset_lines) = config::preset_elements(name) {
                    self.items = config::element_lines_to_bar_items(preset_lines);
                }
            }
        }
        if !self.items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    fn is_editing(&self) -> bool {
        self.budget_editing || self.separator_editing
    }

    fn open_picker(&mut self) {
        let ctx = self.demo_render_ctx();

        let mut available: Vec<PickerEntry> = ALL_ELEMENT_NAMES
            .iter()
            .filter_map(|&name| {
                let element = config::parse_element(name)?;
                let is_stats = STATS_ELEMENT_NAMES.contains(&name);
                if is_stats && !self.stats_enabled {
                    return None;
                }
                let rendered = render::render(element, &ctx.input, ctx.icon_mode, &ctx.config, &ctx.agg_stats);
                let preview = rendered
                    .as_deref()
                    .unwrap_or("(no preview)")
                    .into_text()
                    .unwrap_or_default();
                Some(PickerEntry {
                    item: BarItem::Element(element),
                    preview,
                    name,
                    description: element.description(),
                })
            })
            .collect();

        available.push(PickerEntry {
            item: BarItem::LineBreak,
            preview: "─── line break ───"
                .into_text()
                .unwrap_or_default(),
            name: "line_break",
            description: "Split bar into multiple lines",
        });

        let mut list_state = ListState::default();
        if !available.is_empty() {
            list_state.select(Some(0));
        }

        self.picker = Some(PickerState { available, list_state });
    }

    fn handle_event(&mut self, ev: Event) {
        let Event::Key(key) = ev else { return };
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('s') => {
                    self.saved = true;
                    self.should_quit = true;
                    return;
                }
                KeyCode::Char('c') => {
                    self.should_quit = true;
                    return;
                }
                _ => {}
            }
        }

        if self.confirm_quit {
            match key.code {
                KeyCode::Char('s') => {
                    self.saved = true;
                    self.should_quit = true;
                }
                KeyCode::Char('q') | KeyCode::Char('n') => {
                    self.should_quit = true;
                }
                KeyCode::Esc => {
                    self.confirm_quit = false;
                }
                _ => {}
            }
            return;
        }

        if self.picker.is_some() {
            self.handle_picker_input(key);
            return;
        }

        if !self.is_editing() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    if self.focus == Focus::Settings {
                        self.focus = Focus::BarList;
                        return;
                    }
                    self.confirm_quit = true;
                    return;
                }
                KeyCode::Tab => {
                    self.focus = match self.focus {
                        Focus::BarList => Focus::Settings,
                        Focus::Settings => Focus::BarList,
                    };
                    return;
                }
                _ => {}
            }
        }

        let changed = match self.focus {
            Focus::BarList => self.handle_bar_list_input(key),
            Focus::Settings => self.handle_settings_input(key),
        };

        if changed {
            self.rebuild_preview();
        }
    }

    fn handle_bar_list_input(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => {
                self.move_item(true);
                self.sync_layout_from_items();
                true
            }
            KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => {
                self.move_item(false);
                self.sync_layout_from_items();
                true
            }
            KeyCode::Up => {
                self.navigate_list(true);
                false
            }
            KeyCode::Down => {
                self.navigate_list(false);
                false
            }
            KeyCode::Char('a') => {
                self.open_picker();
                false
            }
            KeyCode::Char('d') | KeyCode::Delete | KeyCode::Backspace => {
                self.remove_selected();
                self.sync_layout_from_items();
                true
            }
            KeyCode::Char('b') => {
                self.insert_line_break();
                self.sync_layout_from_items();
                true
            }
            _ => false,
        }
    }

    fn navigate_list(&mut self, up: bool) {
        if self.items.is_empty() { return; }
        let i = self.list_state.selected().unwrap_or(0);
        let new = cycle_index(i, self.items.len(), up);
        self.list_state.select(Some(new));
    }

    fn move_item(&mut self, up: bool) {
        if self.items.len() < 2 { return; }
        let Some(i) = self.list_state.selected() else { return };
        let j = if up {
            if i == 0 { return; }
            i - 1
        } else {
            if i >= self.items.len() - 1 { return; }
            i + 1
        };
        self.items.swap(i, j);
        self.list_state.select(Some(j));
    }

    fn remove_selected(&mut self) {
        let Some(i) = self.list_state.selected() else { return };
        if i >= self.items.len() { return; }
        self.items.remove(i);
        if self.items.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(i.min(self.items.len() - 1)));
        }
    }

    fn insert_line_break(&mut self) {
        let pos = self.list_state.selected().map(|i| i + 1).unwrap_or(self.items.len());
        self.items.insert(pos, BarItem::LineBreak);
        self.list_state.select(Some(pos));
    }

    fn handle_picker_input(&mut self, key: KeyEvent) {
        let Some(ref mut picker) = self.picker else { return };

        match key.code {
            KeyCode::Up => {
                if picker.available.is_empty() { return; }
                let i = picker.list_state.selected().unwrap_or(0);
                let new = cycle_index(i, picker.available.len(), true);
                picker.list_state.select(Some(new));
            }
            KeyCode::Down => {
                if picker.available.is_empty() { return; }
                let i = picker.list_state.selected().unwrap_or(0);
                let new = cycle_index(i, picker.available.len(), false);
                picker.list_state.select(Some(new));
            }
            KeyCode::Enter => {
                if let Some(sel) = picker.list_state.selected() {
                    let item = picker.available[sel].item;
                    let pos = self.list_state.selected().map(|i| i + 1).unwrap_or(self.items.len());
                    self.items.insert(pos, item);
                    self.list_state.select(Some(pos));
                    self.sync_layout_from_items();
                }
                self.picker = None;
                self.rebuild_preview();
            }
            KeyCode::Esc => {
                self.picker = None;
            }
            _ => {}
        }
    }

    fn handle_settings_input(&mut self, key: KeyEvent) -> bool {
        let field = self.settings_field;

        match field {
            0 => self.handle_layout_field(key),
            1 => self.handle_stats_field(key),
            2 if self.stats_enabled => self.handle_budget_field(key),
            f => {
                let offset = if self.stats_enabled { f } else { f + 1 };
                match offset {
                    3 => self.handle_icon_set_field(key),
                    4 => self.handle_separator_field(key),
                    5 => self.handle_bar_style_field(key),
                    _ => false,
                }
            }
        }
    }

    fn handle_layout_field(&mut self, key: KeyEvent) -> bool {
        let visible_count = self.layout_visible_count();
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                self.layout_idx = cycle_index(self.layout_idx, visible_count, key.code == KeyCode::Left);
                self.apply_preset();
                true
            }
            KeyCode::Up | KeyCode::Down => {
                self.navigate_settings(key.code == KeyCode::Up);
                false
            }
            _ => false,
        }
    }

    fn layout_visible_count(&self) -> usize {
        if self.loaded_items.is_some() {
            LAYOUT_NAMES.len()
        } else {
            LAYOUT_NAMES.len() - 1 // hide "loaded"
        }
    }

    fn handle_stats_field(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Left | KeyCode::Right => {
                self.stats_enabled = !self.stats_enabled;
                if !self.stats_enabled {
                    self.items.retain(|item| {
                        match item {
                            BarItem::Element(e) => !STATS_ELEMENT_NAMES.contains(&e.name()),
                            _ => true,
                        }
                    });
                    self.sync_layout_from_items();
                    if self.settings_field == 2 {
                        self.settings_field = 1;
                    }
                }
                true
            }
            KeyCode::Up | KeyCode::Down => {
                self.navigate_settings(key.code == KeyCode::Up);
                false
            }
            _ => false,
        }
    }

    fn handle_budget_field(&mut self, key: KeyEvent) -> bool {
        if self.budget_editing {
            handle_text_edit(key, &mut self.budget_text, &mut self.budget_editing, |c| c.is_ascii_digit() || c == '.')
        } else {
            match key.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.budget_editing = true;
                    false
                }
                KeyCode::Up | KeyCode::Down => {
                    self.navigate_settings(key.code == KeyCode::Up);
                    false
                }
                _ => false,
            }
        }
    }

    fn handle_icon_set_field(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                self.icon_set_idx = cycle_index(self.icon_set_idx, ICON_SET_NAMES.len(), key.code == KeyCode::Left);
                true
            }
            KeyCode::Up | KeyCode::Down => {
                self.navigate_settings(key.code == KeyCode::Up);
                false
            }
            _ => false,
        }
    }

    fn handle_separator_field(&mut self, key: KeyEvent) -> bool {
        if self.separator_editing {
            handle_text_edit(key, &mut self.separator, &mut self.separator_editing, |_| true)
        } else {
            match key.code {
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.separator_editing = true;
                    false
                }
                KeyCode::Up | KeyCode::Down => {
                    self.navigate_settings(key.code == KeyCode::Up);
                    false
                }
                _ => false,
            }
        }
    }

    fn handle_bar_style_field(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                self.bar_style_idx = cycle_index(self.bar_style_idx, BAR_STYLE_NAMES.len(), key.code == KeyCode::Left);
                true
            }
            KeyCode::Up | KeyCode::Down => {
                self.navigate_settings(key.code == KeyCode::Up);
                false
            }
            _ => false,
        }
    }

    fn navigate_settings(&mut self, up: bool) {
        let count = settings_field_count(self.stats_enabled);
        self.settings_field = cycle_index(self.settings_field, count, up);
    }

    fn render_ui(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if area.width < 60 || area.height < 15 {
            let msg = Paragraph::new("Terminal too small.\nNeed at least 60x15.")
                .style(Style::default().fg(Color::Red));
            frame.render_widget(msg, area);
            return;
        }

        let preview_height = (self.preview_text.lines.len() as u16 + 2).max(3).min(area.height / 3);

        let [main_area, preview_area, help_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(preview_height),
            Constraint::Length(1),
        ])
        .areas(area);

        let [settings_area, bar_list_area] = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .areas(main_area);

        self.render_settings_panel(frame, settings_area);
        self.render_bar_list_panel(frame, bar_list_area);
        self.render_preview_panel(frame, preview_area);
        self.render_help(frame, help_area);

        if self.picker.is_some() {
            self.render_picker_overlay(frame, bar_list_area);
        }
    }

    fn render_settings_panel(&self, frame: &mut Frame, area: Rect) {
        let active = self.focus == Focus::Settings;
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Settings ")
            .border_style(section_border_style(active));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let count = settings_field_count(self.stats_enabled);
        let constraints: Vec<Constraint> = (0..count)
            .map(|_| Constraint::Length(2))
            .chain(std::iter::once(Constraint::Fill(1)))
            .collect();
        let rows = Layout::vertical(constraints).split(inner);

        let mut row = 0;

        // Layout
        let is_active = active && self.settings_field == row;
        let layout_name = LAYOUT_NAMES.get(self.layout_idx).unwrap_or(&"custom");
        self.render_radio_inline(frame, rows[row], "Layout", layout_name, is_active);
        row += 1;

        // Stats
        let is_active = active && self.settings_field == row;
        self.render_toggle_field(frame, rows[row], "Stats", self.stats_enabled, is_active);
        row += 1;

        // Budget (only if stats enabled)
        if self.stats_enabled {
            let is_active = active && self.settings_field == row;
            self.render_text_field(
                frame, rows[row], "Budget", &format!("${}", self.budget_text),
                self.budget_editing, is_active,
            );
            row += 1;
        }

        // Icon set
        let is_active = active && self.settings_field == row;
        self.render_radio_inline(frame, rows[row], "Icon set", ICON_SET_NAMES[self.icon_set_idx], is_active);
        row += 1;

        // Separator
        let is_active = active && self.settings_field == row;
        self.render_text_field(
            frame, rows[row], "Separator", &format!("{:?}", self.separator),
            self.separator_editing, is_active,
        );
        row += 1;

        // Bar style
        let is_active = active && self.settings_field == row;
        self.render_radio_inline(frame, rows[row], "Bar style", BAR_STYLE_NAMES[self.bar_style_idx], is_active);
    }

    fn build_bar_list_items(&self, ctx: &DemoRenderCtx) -> Vec<ListItem<'static>> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let num = format!("{:>2}. ", i + 1);
                match item {
                    BarItem::Element(e) => {
                        let preview_str = render_item_preview(item, ctx)
                            .unwrap_or_default();
                        let preview_text = preview_str.into_text().unwrap_or_default();
                        let name_span = Span::styled(
                            format!("  ({} — {})", e.name(), e.description()),
                            Style::default().fg(Color::DarkGray),
                        );

                        let mut spans = vec![Span::raw(num)];
                        if let Some(first_line) = preview_text.lines.into_iter().next() {
                            spans.extend(first_line.spans);
                        }
                        spans.push(name_span);
                        ListItem::new(Line::from(spans))
                    }
                    BarItem::LineBreak => {
                        ListItem::new(Line::from(vec![
                            Span::raw(num),
                            Span::styled(
                                "─── line break ───",
                                Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
                            ),
                        ]))
                    }
                }
            })
            .collect()
    }

    fn render_bar_list_panel(&mut self, frame: &mut Frame, area: Rect) {
        let active = self.focus == Focus::BarList && self.picker.is_none();
        let count = self.items.len();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Bar layout ({count} items) "))
            .border_style(section_border_style(active));

        let list = List::new(self.bar_list_cache.clone())
            .block(block)
            .highlight_style(highlight_style());
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_picker_overlay(&mut self, frame: &mut Frame, area: Rect) {
        let Some(ref mut picker) = self.picker else { return };

        let overlay = centered_rect(80, 80, area);

        frame.render_widget(ratatui::widgets::Clear, overlay);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Add element (↑↓:navigate  Enter:add  Esc:cancel) ")
            .border_style(Style::default().fg(Color::Yellow));

        let items: Vec<ListItem> = picker.available
            .iter()
            .map(|entry| {
                let mut spans = Vec::new();
                if let Some(first_line) = entry.preview.lines.first() {
                    spans.extend(first_line.spans.clone());
                }
                spans.push(Span::styled(
                    format!("  {} — {}", entry.name, entry.description),
                    Style::default().fg(Color::DarkGray),
                ));
                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());
        frame.render_stateful_widget(list, overlay, &mut picker.list_state);
    }

    fn render_preview_panel(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Preview (sample data) ");
        let paragraph = Paragraph::new(self.preview_text.clone())
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(paragraph, area);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        if self.confirm_quit {
            let line = Line::from(vec![
                Span::styled("Save and quit? ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled("s:save  q/n:quit without saving  Esc:cancel", Style::default().fg(Color::DarkGray)),
            ]);
            frame.render_widget(Paragraph::new(line), area);
            return;
        }
        let help = if self.picker.is_some() {
            "↑↓:navigate  Enter:add  Esc:cancel"
        } else if self.is_editing() {
            "Type: edit  Enter/Tab: confirm  Esc: revert  ^S:save  ^C/q:quit"
        } else {
            match self.focus {
                Focus::BarList => "a:add  d/⌫:del  Shift+↑↓:move  b:break  Tab:settings  ^S:save  ^C/q:quit",
                Focus::Settings => "↑↓:navigate  ←→:cycle  Space:toggle  Tab:bar list  ^S:save  ^C/q:quit  Esc:back",
            }
        };
        frame.render_widget(
            Paragraph::new(help).style(Style::default().fg(Color::DarkGray)),
            area,
        );
    }

    fn render_radio_inline(
        &self, frame: &mut Frame, area: Rect,
        label: &str, value: &str, active: bool,
    ) {
        let (indicator, label_style) = active_indicator(active);
        let line = Line::from(vec![
            field_label(indicator, label, label_style),
            Span::styled(value.to_string(), Style::default().add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_toggle_field(
        &self, frame: &mut Frame, area: Rect,
        label: &str, value: bool, active: bool,
    ) {
        let (indicator, label_style) = active_indicator(active);
        let (check, check_style) = if value {
            ("enabled", Style::default().fg(Color::Green))
        } else {
            ("disabled", Style::default().fg(Color::DarkGray))
        };
        let line = Line::from(vec![
            field_label(indicator, label, label_style),
            Span::styled(check, check_style),
        ]);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn render_text_field(
        &self, frame: &mut Frame, area: Rect,
        label: &str, display: &str, editing: bool, active: bool,
    ) {
        let (indicator, label_style) = active_indicator(active);
        let value_style = if editing {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default().add_modifier(Modifier::BOLD)
        };

        let mut spans = vec![
            field_label(indicator, label, label_style),
            Span::styled(display.to_string(), value_style),
        ];
        if editing {
            spans.push(Span::styled("▎", Style::default().fg(Color::Yellow)));
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);
    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn run() {
    if !std::io::stdin().is_terminal() {
        eprintln!("Error: --configure requires an interactive terminal.");
        std::process::exit(1);
    }

    let existing = {
        let path = toml_config::default_config_path();
        if path.exists() {
            toml_config::load_config(Some(path.to_str().unwrap()))
        } else {
            None
        }
    };

    let mut terminal = ratatui::init();
    let mut app = App::new(existing);

    loop {
        terminal
            .draw(|frame| app.render_ui(frame))
            .expect("failed to draw");

        if let Ok(ev) = event::read() {
            app.handle_event(ev);
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();

    if app.saved {
        let config = app.build_config();
        let path = toml_config::default_config_path();

        let toml_str = toml::to_string_pretty(&config).unwrap_or_else(|e| {
            eprintln!("Error serializing config: {e}");
            std::process::exit(1);
        });

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Error creating directory {}: {e}", parent.display());
                std::process::exit(1);
            });
        }

        std::fs::write(&path, toml_str).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {e}", path.display());
            std::process::exit(1);
        });

        eprintln!("Saved to {}", path.display());
    } else {
        eprintln!("Quit without saving.");
    }
}
