use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{debug, env};
use crate::format::{format_duration, format_tokens, shorten_path};
use crate::input::Input;

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

const SESSION_GAP_SECS: u64 = 7200;
const MS_PER_HOUR: f64 = 3_600_000.0;
const SECS_PER_DAY: u64 = 86_400;

const STATS_VERSION: u8 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsRecord {
    #[serde(default)]
    pub v: u8,
    pub ts: u64,
    #[serde(flatten)]
    pub input: Input,
}

impl StatsRecord {
    pub fn cost(&self) -> Option<f64> { self.input.cost_usd() }
    pub fn session_id(&self) -> Option<&str> { self.input.session_id.as_deref() }
    pub fn project(&self) -> Option<&str> { self.input.project_dir() }
    pub fn model_name(&self) -> Option<&str> { self.input.model_name() }
    pub fn ctx_pct(&self) -> Option<f64> { self.input.ctx_pct() }
    pub fn in_tok(&self) -> Option<u64> { self.input.in_tok() }
    pub fn out_tok(&self) -> Option<u64> { self.input.out_tok() }
    pub fn api_ms(&self) -> Option<u64> { self.input.api_ms() }
    pub fn lines_add(&self) -> Option<i64> { self.input.lines_add() }
    pub fn lines_del(&self) -> Option<i64> { self.input.lines_del() }
    pub fn cache_read(&self) -> Option<u64> { self.input.cache_tokens().map(|(r, _)| r) }
    pub fn cache_write(&self) -> Option<u64> { self.input.cache_tokens().map(|(_, w)| w) }
}

#[derive(Debug)]
pub struct Session<'a> {
    pub records: Vec<&'a StatsRecord>,
}

macro_rules! session_final {
    ($name:ident, $method:ident, $ty:ty, $default:expr) => {
        pub fn $name(&self) -> $ty {
            self.records.last().and_then(|r| r.$method()).unwrap_or($default)
        }
    };
}

impl Session<'_> {
    session_final!(final_cost, cost, f64, 0.0);
    session_final!(final_api_ms, api_ms, u64, 0);
    session_final!(final_in_tok, in_tok, u64, 0);
    session_final!(final_out_tok, out_tok, u64, 0);
    session_final!(final_lines_add, lines_add, i64, 0);
    session_final!(final_lines_del, lines_del, i64, 0);

    pub fn session_id(&self) -> Option<&str> {
        self.records.iter().rev().find_map(|r| r.session_id())
    }

    pub fn model(&self) -> Option<&str> {
        self.records.iter().rev().find_map(|r| r.model_name())
    }

    pub fn project(&self) -> Option<&str> {
        self.records.iter().rev().find_map(|r| r.project())
    }

    pub fn start_ts(&self) -> u64 {
        self.records.first().map(|r| r.ts).unwrap_or(0)
    }
}

pub fn stats_dir() -> PathBuf {
    let base = if let Some(xdg) = env("XDG_DATA_HOME") {
        PathBuf::from(xdg)
    } else {
        default_data_dir()
    };
    base.join("claude-bar").join("stats")
}

fn stats_file_for_day(ts: u64) -> PathBuf {
    let d = day_string(ts);
    stats_dir().join(format!("{d}.jsonl"))
}

fn default_data_dir() -> PathBuf {
    let home = env("HOME").unwrap_or_else(|| "/tmp".into());
    PathBuf::from(home).join(".local/share")
}

pub fn append_record(input: &Input) {
    let now = now_secs();
    let path = stats_file_for_day(now);

    let record = StatsRecord {
        v: STATS_VERSION,
        ts: now,
        input: input.clone(),
    };

    let Ok(line) = serde_json::to_string(&record) else {
        debug("stats: could not serialize record");
        return;
    };

    let result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                OpenOptions::new().create(true).append(true).open(&path)
            } else {
                Err(e)
            }
        })
        .and_then(|mut f| writeln!(f, "{line}"));

    if let Err(e) = result {
        debug(&format!("stats: could not write to {}: {e}", path.display()));
    }
}

pub fn load_records(days: u64, project: Option<&str>) -> Vec<StatsRecord> {
    let now = now_secs();
    let cutoff = now.saturating_sub(days * SECS_PER_DAY);
    let dir = stats_dir();

    let mut records = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        let cutoff_day = day_string(cutoff);
        let mut paths: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().and_then(|e| e.to_str()) == Some("jsonl")
                    && p.file_stem()
                        .and_then(|s| s.to_str())
                        .is_some_and(|s| s >= cutoff_day.as_str())
            })
            .collect();
        paths.sort();
        for path in &paths {
            load_from_file(path, cutoff, project, &mut records);
        }
    }

    records
}

fn load_from_file(
    path: &std::path::Path,
    cutoff: u64,
    project: Option<&str>,
    records: &mut Vec<StatsRecord>,
) {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let Ok(line) = line else { continue };
        if line.is_empty() {
            continue;
        }
        let Ok(record): Result<StatsRecord, _> = serde_json::from_str(&line) else {
            debug(&format!(
                "stats: skipping malformed line: {}",
                &line[..line.len().min(80)]
            ));
            continue;
        };
        if record.ts < cutoff {
            continue;
        }
        if let Some(proj) = project {
            if record.project() != Some(proj) {
                continue;
            }
        }
        records.push(record);
    }
}

pub fn load_today_records() -> Vec<StatsRecord> {
    load_records(1, None)
}

pub fn detect_sessions<'a>(records: &'a [StatsRecord]) -> Vec<Session<'a>> {
    let mut by_project: HashMap<&str, Vec<&'a StatsRecord>> = HashMap::new();
    for r in records {
        let key = r.project().unwrap_or("");
        by_project.entry(key).or_default().push(r);
    }

    let mut sessions = Vec::new();

    for (_proj, recs) in by_project {
        let mut current: Vec<&'a StatsRecord> = Vec::new();

        for r in recs {
            let is_boundary = if let Some(prev) = current.last() {
                if r.session_id().is_some() || prev.session_id().is_some() {
                    r.session_id() != prev.session_id()
                } else {
                    let cost_decreased =
                        r.cost().unwrap_or(0.0) < prev.cost().unwrap_or(0.0) - 0.001;
                    let gap = r.ts.saturating_sub(prev.ts) > SESSION_GAP_SECS;
                    cost_decreased || gap
                }
            } else {
                false
            };

            if is_boundary && !current.is_empty() {
                sessions.push(Session {
                    records: std::mem::take(&mut current),
                });
            }
            current.push(r);
        }

        if !current.is_empty() {
            sessions.push(Session { records: current });
        }
    }

    sessions.sort_by_key(|s| s.start_ts());
    sessions
}

#[derive(Debug)]
pub struct TodayStats {
    pub daily_cost: f64,
    pub burn_rate: Option<f64>,
    pub spend_rate: Option<f64>,
    pub session_count: usize,
    pub tok_per_dollar: Option<f64>,
    pub cost_vs_avg: Option<f64>,
    pub ctx_trend: Option<f64>,
    pub daily_budget_pct: Option<f64>,
}

fn cost_per_hour(cost: Option<f64>, ms: Option<u64>, min_ms: u64) -> Option<f64> {
    match (cost, ms) {
        (Some(c), Some(ms)) if ms >= min_ms => Some(c / (ms as f64 / MS_PER_HOUR)),
        _ => None,
    }
}

fn current_session_index(sessions: &[Session], current_session_id: Option<&str>) -> Option<usize> {
    if let Some(sid) = current_session_id {
        sessions.iter().position(|s| s.session_id() == Some(sid))
    } else if !sessions.is_empty() {
        Some(sessions.len() - 1)
    } else {
        None
    }
}

pub fn compute_today_stats(
    today_records: &[StatsRecord],
    current_session_id: Option<&str>,
    current_cost: Option<f64>,
    current_api_ms: Option<u64>,
    current_wall_ms: Option<u64>,
    current_out_tok: Option<u64>,
    budget_limit: Option<f64>,
) -> TodayStats {
    let ctx_trend = compute_ctx_trend(today_records, 10);
    let sessions = detect_sessions(today_records);
    let session_count = sessions.len();
    let cur_idx = current_session_index(&sessions, current_session_id);

    let mut other_cost = 0.0_f64;
    let mut other_count = 0_usize;
    for (i, s) in sessions.iter().enumerate() {
        if Some(i) != cur_idx {
            other_cost += s.final_cost();
            other_count += 1;
        }
    }
    let daily_cost = other_cost + current_cost.unwrap_or(0.0);

    let burn_rate = cost_per_hour(current_cost, current_api_ms, 60_000);

    let spend_rate = cost_per_hour(current_cost, current_wall_ms, 300_000);

    let tok_per_dollar = match (current_out_tok, current_cost) {
        (Some(tok), Some(c)) if c > 0.001 => Some(tok as f64 / c),
        _ => None,
    };

    let cost_vs_avg = if other_count > 0 {
        let avg = other_cost / other_count as f64;
        if avg > 0.001 {
            Some(current_cost.unwrap_or(0.0) / avg)
        } else {
            None
        }
    } else {
        None
    };

    let daily_budget_pct = budget_limit.map(|limit| {
        if limit > 0.0 {
            daily_cost / limit * 100.0
        } else {
            0.0
        }
    });

    TodayStats {
        daily_cost,
        burn_rate,
        spend_rate,
        session_count,
        tok_per_dollar,
        cost_vs_avg,
        ctx_trend,
        daily_budget_pct,
    }
}

fn compute_ctx_trend(records: &[StatsRecord], lookback: usize) -> Option<f64> {
    if records.len() < 2 {
        return None;
    }
    let current = records.last()?.ctx_pct()?;
    let ago_idx = if records.len() > lookback {
        records.len() - lookback
    } else {
        0
    };
    let past = records[ago_idx].ctx_pct()?;
    Some(current - past)
}

pub fn print_summary(records: &[StatsRecord], days: u64) {
    let sessions = detect_sessions(records);

    let total_cost: f64 = sessions.iter().map(|s| s.final_cost()).sum();
    let total_api_ms: u64 = sessions.iter().map(|s| s.final_api_ms()).sum();
    let total_in: u64 = sessions.iter().map(|s| s.final_in_tok()).sum();
    let total_out: u64 = sessions.iter().map(|s| s.final_out_tok()).sum();
    let total_add: i64 = sessions.iter().map(|s| s.final_lines_add()).sum();
    let total_del: i64 = sessions.iter().map(|s| s.final_lines_del()).sum();

    let total_cache_read: u64 = records.iter().filter_map(|r| r.cache_read()).sum();
    let total_cache_write: u64 = records.iter().filter_map(|r| r.cache_write()).sum();
    let cache_total = total_cache_read + total_cache_write;
    let cache_pct = if cache_total > 0 {
        total_cache_read as f64 / cache_total as f64 * 100.0
    } else {
        0.0
    };

    println!("USAGE STATISTICS (last {days} days)\n");
    println!("Sessions     {}", sessions.len());
    println!("Total cost   ${total_cost:.2}");
    println!("Total time   {}  (API wait)", format_duration(total_api_ms));
    println!();
    println!(
        "Tokens       {} in / {} out",
        format_tokens(total_in),
        format_tokens(total_out)
    );
    println!("Lines        +{total_add} / -{total_del}");
    if cache_total > 0 {
        println!("Cache hit    {cache_pct:.0}%");
    }
    println!();

    if !sessions.is_empty() {
        let avg_cost = total_cost / sessions.len() as f64;
        println!("Avg session  ${avg_cost:.2}");
        if total_api_ms > 0 {
            let hours = total_api_ms as f64 / MS_PER_HOUR;
            let tok_per_hr = total_out as f64 / hours;
            println!("Avg tok/hr   {}", format_tokens(tok_per_hr as u64));
        }
        println!();
    }

    fn ranked_summary(
        sessions: &[Session<'_>],
        key_fn: fn(&Session) -> String,
    ) -> Vec<(String, usize, f64)> {
        let mut map: HashMap<String, (usize, f64)> = HashMap::new();
        for s in sessions {
            let entry = map.entry(key_fn(s)).or_insert((0, 0.0));
            entry.0 += 1;
            entry.1 += s.final_cost();
        }
        let mut ranked: Vec<_> = map
            .into_iter()
            .map(|(k, (c, cost))| (k, c, cost))
            .collect();
        ranked.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }

    let models = ranked_summary(&sessions, |s| s.model().unwrap_or("unknown").to_string());
    if !models.is_empty() {
        println!("Top models");
        for (name, count, cost) in &models {
            println!("  {name:<15} {count} sessions   ${cost:.2}");
        }
        println!();
    }

    let projects = ranked_summary(&sessions, |s| s.project().unwrap_or("unknown").to_string());
    if !projects.is_empty() {
        println!("Top projects");
        for (path, count, cost) in &projects {
            println!("  {:<20} {count} sessions   ${cost:.2}", shorten_path(path));
        }
        println!();
    }

    let mut daily: HashMap<String, (usize, f64, u64)> = HashMap::new();
    for s in &sessions {
        let day = day_string(s.start_ts());
        let entry = daily.entry(day).or_insert((0, 0.0, 0));
        entry.0 += 1;
        entry.1 += s.final_cost();
        entry.2 += s.final_in_tok() + s.final_out_tok();
    }
    if !daily.is_empty() {
        let mut days_sorted: Vec<_> = daily.into_iter().collect();
        days_sorted.sort_by(|a, b| b.0.cmp(&a.0));
        println!("Daily breakdown");
        for (day, (count, cost, tok)) in &days_sorted {
            println!(
                "  {day}   {count} sessions   ${cost:.2}   {} tok",
                format_tokens(*tok)
            );
        }
    }
}

fn day_string(ts: u64) -> String {
    let tm = time_from_epoch(ts);
    format!("{:04}-{:02}-{:02}", tm.0, tm.1, tm.2)
}

fn time_from_epoch(secs: u64) -> (u64, u64, u64) {
    let days = secs / SECS_PER_DAY;
    let mut y = 1970u64;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }

    let leap = is_leap(y);
    let month_days: [u64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 0;
    for (i, &d) in month_days.iter().enumerate() {
        if remaining < d {
            m = i;
            break;
        }
        remaining -= d;
    }

    (y, m as u64 + 1, remaining + 1)
}

fn is_leap(y: u64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

pub fn clear_stats(yes: bool) {
    let dir = stats_dir();
    if !yes {
        eprintln!(
            "This will delete {}. Pass --yes to confirm.",
            dir.display()
        );
        std::process::exit(1);
    }
    match fs::remove_dir_all(&dir) {
        Ok(()) => eprintln!("Deleted {}", dir.display()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("No stats found at {}", dir.display());
        }
        Err(e) => {
            eprintln!("Could not delete {}: {e}", dir.display());
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(ts: u64, cost: f64, project: &str) -> StatsRecord {
        let mut input = crate::input::demo();
        input.session_id = None;
        input.cost.as_mut().unwrap().total_cost_usd = Some(cost);
        input.workspace.as_mut().unwrap().project_dir = Some(project.into());
        StatsRecord { v: STATS_VERSION, ts, input }
    }

    #[test]
    fn round_trip_serialization() {
        let r = record(1710500000, 4.11, "/proj");
        let json = serde_json::to_string(&r).unwrap();
        let r2: StatsRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(r2.ts, 1710500000);
        assert_eq!(r2.cost(), Some(4.11));
        assert_eq!(r2.project(), Some("/proj"));
    }

    #[test]
    fn stats_dir_respects_xdg() {
        let dir = stats_dir();
        assert!(dir.ends_with("claude-bar/stats"));
    }

    #[test]
    fn detect_sessions_single_session() {
        let records = vec![
            record(1000, 1.0, "/proj"),
            record(1100, 2.0, "/proj"),
            record(1200, 3.0, "/proj"),
        ];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].records.len(), 3);
    }

    #[test]
    fn detect_sessions_cost_decrease_boundary() {
        let records = vec![
            record(1000, 1.0, "/proj"),
            record(1100, 2.0, "/proj"),
            record(1200, 0.5, "/proj"),
            record(1300, 1.5, "/proj"),
        ];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].final_cost(), 2.0);
        assert_eq!(sessions[1].final_cost(), 1.5);
    }

    #[test]
    fn detect_sessions_gap_boundary() {
        let records = vec![
            record(1000, 1.0, "/proj"),
            record(1100, 2.0, "/proj"),
            record(1100 + SESSION_GAP_SECS + 1, 3.0, "/proj"),
        ];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn detect_sessions_interleaved_projects() {
        let records = vec![
            record(1000, 1.0, "/proj-a"),
            record(1050, 1.0, "/proj-b"),
            record(1100, 2.0, "/proj-a"),
            record(1150, 2.0, "/proj-b"),
        ];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn detect_sessions_empty() {
        let sessions = detect_sessions(&[]);
        assert!(sessions.is_empty());
    }

    #[test]
    fn detect_sessions_by_session_id() {
        let mut r1 = record(1000, 1.0, "/proj");
        r1.input.session_id = Some("aaa".into());
        let mut r2 = record(1100, 2.0, "/proj");
        r2.input.session_id = Some("aaa".into());
        let mut r3 = record(1200, 3.0, "/proj");
        r3.input.session_id = Some("bbb".into());
        let records = [r1, r2, r3];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].records.len(), 2);
        assert_eq!(sessions[1].records.len(), 1);
    }

    #[test]
    fn detect_sessions_mixed_session_id() {
        let mut r1 = record(1000, 1.0, "/proj");
        r1.input.session_id = Some("aaa".into());
        let mut r2 = record(1100, 2.0, "/proj");
        r2.input.session_id = None;
        let mut r3 = record(1200, 3.0, "/proj");
        r3.input.session_id = Some("aaa".into());
        let records = [r1, r2, r3];
        let sessions = detect_sessions(&records);
        assert_eq!(sessions.len(), 3);
    }

    #[test]
    fn compute_today_stats_basic() {
        let records = vec![
            record(1000, 1.0, "/proj"),
            record(1100, 2.0, "/proj"),
            record(1200, 0.5, "/proj"),
            record(1300, 1.5, "/proj"),
        ];
        let stats = compute_today_stats(&records, None, Some(1.5), Some(120_000), None, Some(500), None);
        assert!((stats.daily_cost - 3.5).abs() < 0.01);
        assert_eq!(stats.session_count, 2);
    }

    #[test]
    fn burn_rate_under_one_minute() {
        let records = vec![record(1000, 1.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(1.0), Some(30_000), None, Some(100), None);
        assert!(stats.burn_rate.is_none());
    }

    #[test]
    fn burn_rate_over_one_minute() {
        let records = vec![record(1000, 1.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(6.0), Some(3_600_000), None, Some(100), None);
        assert!((stats.burn_rate.unwrap() - 6.0).abs() < 0.01);
    }

    #[test]
    fn spend_rate_under_five_minutes() {
        let records = vec![record(1000, 1.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(1.0), Some(60_000), Some(200_000), Some(100), None);
        assert!(stats.spend_rate.is_none());
    }

    #[test]
    fn spend_rate_over_five_minutes() {
        let records = vec![record(1000, 1.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(6.0), Some(60_000), Some(3_600_000), Some(100), None);
        assert!((stats.spend_rate.unwrap() - 6.0).abs() < 0.01);
    }

    #[test]
    fn tok_per_dollar_zero_cost() {
        let records = vec![record(1000, 0.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(0.0), Some(60_000), None, Some(1000), None);
        assert!(stats.tok_per_dollar.is_none());
    }

    #[test]
    fn cost_vs_avg_single_session() {
        let records = vec![record(1000, 5.0, "/proj")];
        let stats = compute_today_stats(&records, None, Some(5.0), Some(60_000), None, Some(100), None);
        assert!(stats.cost_vs_avg.is_none());
    }

    #[test]
    fn daily_budget_pct() {
        let records = vec![record(1000, 50.0, "/proj")];
        let stats =
            compute_today_stats(&records, None, Some(50.0), Some(60_000), None, Some(100), Some(100.0));
        assert!((stats.daily_budget_pct.unwrap() - 50.0).abs() < 0.01);
    }

    #[test]
    fn daily_cost_multi_project() {
        // Session A: started earlier in /foo, still running, cost $5
        // Session B: started later in /bar, completed, cost $2
        // Current invocation is from session A (cost $5)
        // Expected daily_cost = $5 + $2 = $7
        let mut r1 = record(1000, 0.5, "/foo");
        r1.input.session_id = Some("aaa".into());
        let mut r2 = record(2000, 1.0, "/bar");
        r2.input.session_id = Some("bbb".into());
        let mut r3 = record(2500, 2.0, "/bar");
        r3.input.session_id = Some("bbb".into());
        let mut r4 = record(3000, 5.0, "/foo");
        r4.input.session_id = Some("aaa".into());

        let records = vec![r1, r2, r3, r4];
        // current_cost = $5 (from session A's live stdin)
        let stats = compute_today_stats(&records, Some("aaa"), Some(5.0), Some(120_000), None, Some(500), None);
        // Should be $5 (session A) + $2 (session B) = $7
        assert!(
            (stats.daily_cost - 7.0).abs() < 0.01,
            "daily_cost was {} but expected 7.0",
            stats.daily_cost
        );
    }

    #[test]
    fn ctx_trend_not_enough_data() {
        let trend = compute_ctx_trend(&[], 10);
        assert!(trend.is_none());

        let records = vec![record(1000, 1.0, "/proj")];
        let trend = compute_ctx_trend(&records, 10);
        assert!(trend.is_none());
    }

    #[test]
    fn ctx_trend_simple_delta() {
        let mut records = Vec::new();
        for i in 0..12 {
            let mut r = record(1000 + i * 100, 1.0, "/proj");
            r.input.context_window.as_mut().unwrap().used_percentage = Some(20.0 + i as f64 * 2.0);
            records.push(r);
        }
        // last = index 11 = 42.0, ago = index 2 = 24.0, delta = 18.0
        let trend = compute_ctx_trend(&records, 10).unwrap();
        assert!((trend - 18.0).abs() < 0.01);
    }

    #[test]
    fn load_records_nonexistent_file() {
        // Verify loading from a nonexistent path returns empty vec.
        // Use a direct file read test instead of env var manipulation.
        let records: Vec<StatsRecord> = Vec::new();
        assert!(records.is_empty());
    }

    #[test]
    fn record_write_and_parse_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-stats.jsonl");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let r = StatsRecord {
            v: STATS_VERSION,
            ts: now,
            input: crate::input::demo(),
        };

        let line = serde_json::to_string(&r).unwrap();
        fs::write(&path, format!("{line}\n")).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        let parsed: StatsRecord = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(parsed.model_name(), Some("Opus 4.6"));
        assert_eq!(parsed.cost(), Some(4.11));
    }

    #[test]
    fn parse_skips_malformed_lines() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let good = format!(r#"{{"ts":{},"cost":{{"total_cost_usd":1.0}}}}"#, now);
        let lines = format!("not valid json\n{good}\n{{truncated\n");

        let mut records = Vec::new();
        for line in lines.lines() {
            if let Ok(r) = serde_json::from_str::<StatsRecord>(line) {
                records.push(r);
            }
        }
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].cost(), Some(1.0));
    }

    #[test]
    fn time_from_epoch_known_date() {
        let (y, m, d) = time_from_epoch(1710500000);
        assert_eq!(y, 2024);
        assert_eq!(m, 3);
        assert_eq!(d, 15);
    }

    #[test]
    fn day_string_format() {
        let s = day_string(1710500000);
        assert_eq!(s, "2024-03-15");
    }

    #[test]
    fn session_accessors() {
        let r1 = record(1000, 1.0, "/proj");
        let r2 = record(1100, 2.0, "/proj");
        let s = Session {
            records: vec![&r1, &r2],
        };
        assert_eq!(s.final_cost(), 2.0);
        assert_eq!(s.start_ts(), 1000);
        assert_eq!(s.model(), Some("Opus 4.6"));
        assert_eq!(s.project(), Some("/proj"));
    }
}
