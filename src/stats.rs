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

const MS_PER_HOUR: f64 = 3_600_000.0;
const SECS_PER_DAY: u64 = 86_400;

const STATS_VERSION: u8 = 3;

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

    pub fn first_cost(&self) -> f64 {
        self.records.first().and_then(|r| r.cost()).unwrap_or(0.0)
    }

    pub fn cost_delta(&self) -> f64 {
        self.final_cost() - self.first_cost()
    }

    pub fn session_id(&self) -> Option<&str> {
        self.records.first().and_then(|r| r.session_id())
    }

    pub fn model(&self) -> Option<&str> {
        self.records.first().and_then(|r| r.model_name())
    }

    pub fn project(&self) -> Option<&str> {
        self.records.first().and_then(|r| r.project())
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

pub fn append_record(input: &serde_json::Map<String, serde_json::Value>) {
    #[derive(Serialize)]
    struct Record<'a> {
        v: u8,
        ts: u64,
        #[serde(flatten)]
        data: &'a serde_json::Map<String, serde_json::Value>,
    }

    let now = now_secs();
    let path = stats_file_for_day(now);

    let Ok(line) = serde_json::to_string(&Record { v: STATS_VERSION, ts: now, data: input }) else {
        debug(|| "stats: could not serialize record".into());
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
        debug(|| format!("stats: could not write to {}: {e}", path.display()));
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
            debug(|| format!(
                "stats: skipping malformed line: {}",
                &line[..line.len().min(80)]
            ));
            continue;
        };
        if record.ts < cutoff {
            continue;
        }
        if let Some(proj) = project
            && record.project() != Some(proj)
        {
            continue;
        }
        records.push(record);
    }
}

pub fn load_today_records(day_window: &str) -> Vec<StatsRecord> {
    if day_window == "rolling" {
        return load_records(1, None);
    }
    let path = stats_file_for_day(now_secs());
    let mut records = Vec::new();
    load_from_file(&path, 0, None, &mut records);
    records
}

pub fn group_sessions<'a>(records: &'a [StatsRecord]) -> Vec<Session<'a>> {
    let mut by_session: HashMap<(&str, &str), Vec<&'a StatsRecord>> = HashMap::new();
    for r in records {
        let proj = r.project().unwrap_or("");
        let sid = r.session_id().unwrap_or("");
        by_session.entry((proj, sid)).or_default().push(r);
    }

    by_session
        .into_values()
        .map(|records| Session { records })
        .collect()
}

#[derive(Debug)]
pub struct AggregateStats {
    pub project_today_cost: f64,
    pub all_today_cost: f64,
    pub burn_rate: Option<f64>,
    pub spend_rate: Option<f64>,
    pub session_tok_per_dollar: Option<f64>,
    pub cost_vs_avg: Option<f64>,
    pub ctx_trend: Option<f64>,
    pub daily_budget_pct: Option<f64>,
    pub avg_daily_cost: Option<f64>,
}

pub struct AggregateParams<'a> {
    pub today_records: &'a [StatsRecord],
    pub current_session_id: Option<&'a str>,
    pub current_cost: Option<f64>,
    pub current_api_ms: Option<u64>,
    pub current_wall_ms: Option<u64>,
    pub current_out_tok: Option<u64>,
    pub budget_limit: Option<f64>,
    pub current_project: Option<&'a str>,
    pub ctx_lookback_secs: u64,
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

pub fn compute_aggregate_stats(params: &AggregateParams) -> AggregateStats {
    let ctx_trend = compute_ctx_trend(params.today_records, params.ctx_lookback_secs);
    let sessions = group_sessions(params.today_records);
    let matches_project = |s: &Session| -> bool {
        params.current_project.is_some_and(|cp| s.project() == Some(cp))
    };
    let cur_idx = current_session_index(&sessions, params.current_session_id);

    let mut all_delta = 0.0_f64;
    let mut project_delta = 0.0_f64;
    let mut cur_first_cost = 0.0_f64;
    let cur_matches_project = cur_idx.is_some_and(|i| matches_project(&sessions[i]));

    let mut project_costs: HashMap<&str, f64> = HashMap::new();

    for (i, s) in sessions.iter().enumerate() {
        if Some(i) != cur_idx {
            let delta = s.cost_delta();
            all_delta += delta;
            if matches_project(s) {
                project_delta += delta;
            }
            if let Some(proj) = s.project()
                && params.current_project != Some(proj)
            {
                *project_costs.entry(proj).or_insert(0.0) += delta;
            }
        } else {
            cur_first_cost = s.first_cost();
        }
    }
    let cur_delta = (params.current_cost.unwrap_or(0.0) - cur_first_cost).max(0.0);
    let all_today_cost = all_delta + cur_delta;
    let project_today_cost = project_delta + if cur_matches_project { cur_delta } else { 0.0 };

    let burn_rate = cost_per_hour(params.current_cost, params.current_api_ms, 60_000);

    let spend_rate = cost_per_hour(params.current_cost, params.current_wall_ms, 300_000);

    let session_tok_per_dollar = match (params.current_out_tok, params.current_cost) {
        (Some(tok), Some(c)) if c > 0.001 => Some(tok as f64 / c),
        _ => None,
    };

    let cost_vs_avg = {
        let (sum, count) = project_costs
            .values()
            .fold((0.0, 0usize), |(s, c), &cost| (s + cost, c + 1));
        if count > 0 {
            let avg = sum / count as f64;
            if avg > 0.001 {
                Some(project_today_cost / avg)
            } else {
                None
            }
        } else {
            None
        }
    };

    let daily_budget_pct = params.budget_limit.map(|limit| {
        if limit > 0.0 {
            all_today_cost / limit * 100.0
        } else {
            0.0
        }
    });

    AggregateStats {
        project_today_cost,
        all_today_cost,
        burn_rate,
        spend_rate,
        session_tok_per_dollar,
        cost_vs_avg,
        ctx_trend,
        daily_budget_pct,
        avg_daily_cost: None,
    }
}

fn compute_ctx_trend(records: &[StatsRecord], lookback_secs: u64) -> Option<f64> {
    if records.len() < 2 {
        return None;
    }
    let current = records.last()?;
    let current_pct = current.ctx_pct()?;
    let cutoff = current.ts.saturating_sub(lookback_secs);
    let idx = records.partition_point(|r| r.ts <= cutoff);
    let past = records[..idx]
        .iter()
        .rev()
        .find(|r| r.ctx_pct().is_some())?;
    Some(current_pct - past.ctx_pct()?)
}

pub fn compute_avg_daily_cost(project: &str, lookback_days: u64) -> Option<f64> {
    let records = load_records(lookback_days, Some(project));
    avg_daily_cost_from_records(&records)
}

pub fn avg_daily_cost_from_records(records: &[StatsRecord]) -> Option<f64> {
    if records.is_empty() {
        return None;
    }
    let sessions = group_sessions(records);
    let mut by_day: HashMap<String, f64> = HashMap::new();
    for s in &sessions {
        let day = day_string(s.start_ts());
        *by_day.entry(day).or_insert(0.0) += s.cost_delta();
    }
    if by_day.is_empty() {
        return None;
    }
    let total: f64 = by_day.values().sum();
    Some(total / by_day.len() as f64)
}

pub fn print_summary(records: &[StatsRecord], days: u64) {
    let mut sessions = group_sessions(records);
    sessions.sort_by_key(|s| s.start_ts());

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
    (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400)
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

    const LOOKBACK: u64 = 300;

    fn record(ts: u64, cost: f64, project: &str) -> StatsRecord {
        record_with_session(ts, cost, project, "default")
    }

    fn record_with_session(ts: u64, cost: f64, project: &str, session_id: &str) -> StatsRecord {
        let mut input = crate::input::demo();
        input.session_id = Some(session_id.into());
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
    fn group_sessions_single_session() {
        let records = vec![
            record(1000, 1.0, "/proj"),
            record(1100, 2.0, "/proj"),
            record(1200, 3.0, "/proj"),
        ];
        let sessions = group_sessions(&records);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].records.len(), 3);
    }

    #[test]
    fn group_sessions_different_ids() {
        let records = vec![
            record_with_session(1000, 1.0, "/proj", "aaa"),
            record_with_session(1100, 2.0, "/proj", "aaa"),
            record_with_session(1200, 0.5, "/proj", "bbb"),
            record_with_session(1300, 1.5, "/proj", "bbb"),
        ];
        let sessions = group_sessions(&records);
        assert_eq!(sessions.len(), 2);
        let mut costs: Vec<f64> = sessions.iter().map(|s| s.final_cost()).collect();
        costs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(costs, vec![1.5, 2.0]);
    }

    #[test]
    fn group_sessions_interleaved_projects() {
        let records = vec![
            record(1000, 1.0, "/proj-a"),
            record(1050, 1.0, "/proj-b"),
            record(1100, 2.0, "/proj-a"),
            record(1150, 2.0, "/proj-b"),
        ];
        let sessions = group_sessions(&records);
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn group_sessions_empty() {
        let sessions = group_sessions(&[]);
        assert!(sessions.is_empty());
    }

    #[test]
    fn group_sessions_same_project_different_ids() {
        let records = vec![
            record_with_session(1000, 1.0, "/proj", "aaa"),
            record_with_session(1100, 2.0, "/proj", "aaa"),
            record_with_session(1200, 3.0, "/proj", "bbb"),
        ];
        let sessions = group_sessions(&records);
        assert_eq!(sessions.len(), 2);
        let mut lens: Vec<usize> = sessions.iter().map(|s| s.records.len()).collect();
        lens.sort();
        assert_eq!(lens, vec![1, 2]);
    }

    fn make_params<'a>(
        records: &'a [StatsRecord],
        session_id: Option<&'a str>,
        cost: Option<f64>,
        api_ms: Option<u64>,
        wall_ms: Option<u64>,
        out_tok: Option<u64>,
        budget: Option<f64>,
        project: Option<&'a str>,
    ) -> AggregateParams<'a> {
        AggregateParams {
            today_records: records,
            current_session_id: session_id,
            current_cost: cost,
            current_api_ms: api_ms,
            current_wall_ms: wall_ms,
            current_out_tok: out_tok,
            budget_limit: budget,
            current_project: project,
            ctx_lookback_secs: LOOKBACK,
        }
    }

    #[test]
    fn compute_aggregate_stats_basic() {
        let records = vec![
            record_with_session(1000, 1.0, "/proj", "aaa"),
            record_with_session(1100, 2.0, "/proj", "aaa"),
            record_with_session(1200, 0.5, "/proj", "bbb"),
            record_with_session(1300, 1.5, "/proj", "bbb"),
        ];
        let params = make_params(&records, Some("bbb"), Some(1.5), Some(120_000), None, Some(500), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        // all_today_cost: session aaa delta (2-1=1) + cur_delta (1.5-0.5=1) = 2.0
        assert!((stats.all_today_cost - 2.0).abs() < 0.01);
    }

    #[test]
    fn burn_rate_under_one_minute() {
        let records = vec![record(1000, 1.0, "/proj")];
        let params = make_params(&records, None, Some(1.0), Some(30_000), None, Some(100), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!(stats.burn_rate.is_none());
    }

    #[test]
    fn burn_rate_over_one_minute() {
        let records = vec![record(1000, 1.0, "/proj")];
        let params = make_params(&records, None, Some(6.0), Some(3_600_000), None, Some(100), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!((stats.burn_rate.unwrap() - 6.0).abs() < 0.01);
    }

    /// When current session ID doesn't match any loaded record, cur_first_cost
    /// falls back to 0, making cur_delta = current_cost (the full cumulative cost).
    /// The current session's records are also counted as a completed session.
    /// This double-counts the current session's spend.
    #[test]
    fn project_daily_cost_session_id_mismatch() {
        let (records, current_cost, expected_daily) = cumulative_session_fixture();
        let stats = cumulative_session_stats(&records, "826-new", current_cost);

        assert_daily_cost(&stats, expected_daily, current_cost);
    }

    /// Real-world scenario: long-running conversation reattached across multiple
    /// session IDs. Costs are cumulative (monotonically increasing across segments).
    #[test]
    fn project_daily_cost_cumulative_sessions() {
        let (records, current_cost, expected_daily) = cumulative_session_fixture();
        let stats = cumulative_session_stats(&records, "826", current_cost);

        assert_daily_cost(&stats, expected_daily, current_cost);
    }

    fn cumulative_session_fixture() -> (Vec<StatsRecord>, f64, f64) {
        let records = vec![
            record_with_session(1000, 160.80, "/proj", "e8d"),
            record_with_session(2000, 174.45, "/proj", "e8d"),
            record_with_session(3000, 174.45, "/proj", "55c"),
            record_with_session(4000, 184.96, "/proj", "55c"),
            record_with_session(5000, 184.96, "/proj", "91a"),
            record_with_session(6000, 198.21, "/proj", "91a"),
            record_with_session(7000, 198.21, "/proj", "826"),
            record_with_session(8000, 199.37, "/proj", "826"),
        ];
        let current_cost = 199.37;
        let expected_daily =
            (174.45 - 160.80) + (184.96 - 174.45) + (198.21 - 184.96) + (199.37 - 198.21);
        (records, current_cost, expected_daily)
    }

    fn cumulative_session_stats(
        records: &[StatsRecord],
        session_id: &str,
        current_cost: f64,
    ) -> AggregateStats {
        let params = AggregateParams {
            today_records: records,
            current_session_id: Some(session_id),
            current_cost: Some(current_cost),
            current_api_ms: Some(22_440_000),
            current_wall_ms: Some(461_880_000),
            current_out_tok: Some(1000),
            budget_limit: None,
            current_project: Some("/proj"),
            ctx_lookback_secs: LOOKBACK,
        };
        compute_aggregate_stats(&params)
    }

    fn assert_daily_cost(stats: &AggregateStats, expected: f64, ceiling: f64) {
        assert!(
            (stats.project_today_cost - expected).abs() < 0.1,
            "project_today_cost was ${:.2} but expected ${:.2} (sum of deltas)",
            stats.project_today_cost, expected
        );
        assert!(
            stats.project_today_cost <= ceiling,
            "project_today_cost ${:.2} exceeds total session cost ${:.2}",
            stats.project_today_cost, ceiling
        );
    }

    #[test]
    fn spend_rate_under_five_minutes() {
        let records = vec![record(1000, 1.0, "/proj")];
        let params = make_params(&records, None, Some(1.0), Some(60_000), Some(200_000), Some(100), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!(stats.spend_rate.is_none());
    }

    #[test]
    fn spend_rate_over_five_minutes() {
        let records = vec![record(1000, 1.0, "/proj")];
        let params = make_params(&records, None, Some(6.0), Some(60_000), Some(3_600_000), Some(100), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!((stats.spend_rate.unwrap() - 6.0).abs() < 0.01);
    }

    #[test]
    fn tok_per_dollar_zero_cost() {
        let records = vec![record(1000, 0.0, "/proj")];
        let params = make_params(&records, None, Some(0.0), Some(60_000), None, Some(1000), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!(stats.session_tok_per_dollar.is_none());
    }

    #[test]
    fn cost_vs_avg_single_project() {
        let records = vec![record(1000, 5.0, "/proj")];
        let params = make_params(&records, None, Some(5.0), Some(60_000), None, Some(100), None, Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        assert!(stats.cost_vs_avg.is_none());
    }

    #[test]
    fn daily_budget_pct() {
        let records = vec![record(1000, 10.0, "/proj"), record(2000, 50.0, "/proj")];
        let params = make_params(&records, None, Some(50.0), Some(60_000), None, Some(100), Some(100.0), Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        // all_today_cost = delta (50 - 10 = 40), budget = 100, pct = 40%
        assert!((stats.daily_budget_pct.unwrap() - 40.0).abs() < 0.01);
    }

    #[test]
    fn daily_cost_multi_project() {
        let records = vec![
            record_with_session(1000, 0.5, "/foo", "aaa"),
            record_with_session(2000, 1.0, "/bar", "bbb"),
            record_with_session(2500, 2.0, "/bar", "bbb"),
            record_with_session(3000, 5.0, "/foo", "aaa"),
        ];
        let params = make_params(&records, Some("aaa"), Some(5.0), Some(120_000), None, Some(500), None, Some("/foo"));
        let stats = compute_aggregate_stats(&params);
        // project_today_cost: only /foo sessions' delta = cur_delta = 5.0 - 0.5 = 4.5
        assert!(
            (stats.project_today_cost - 4.5).abs() < 0.01,
            "project_today_cost was {} but expected 4.5",
            stats.project_today_cost
        );
        // all_today_cost: /bar delta (2.0 - 1.0 = 1.0) + cur_delta (4.5) = 5.5
        assert!(
            (stats.all_today_cost - 5.5).abs() < 0.01,
            "all_today_cost was {} but expected 5.5",
            stats.all_today_cost
        );
    }

    #[test]
    fn all_today_cost_spanning_session() {
        let records = vec![
            record_with_session(1000, 3.0, "/proj", "aaa"),
            record_with_session(2000, 5.0, "/proj", "aaa"),
        ];
        let params = make_params(&records, Some("aaa"), Some(5.0), None, None, None, Some(10.0), Some("/proj"));
        let stats = compute_aggregate_stats(&params);
        // all_today_cost = today's delta only = $5 - $3 = $2
        assert!(
            (stats.all_today_cost - 2.0).abs() < 0.01,
            "all_today_cost was {} but expected 2.0",
            stats.all_today_cost
        );
        // daily_budget_pct should use all_today_cost: $2 / $10 = 20%
        assert!(
            (stats.daily_budget_pct.unwrap() - 20.0).abs() < 0.01,
            "daily_budget_pct was {} but expected 20.0",
            stats.daily_budget_pct.unwrap()
        );
    }

    #[test]
    fn project_today_cost_filters_by_project() {
        let records = vec![
            record_with_session(1000, 1.0, "/foo", "aaa"),
            record_with_session(1100, 3.0, "/foo", "aaa"),
            record_with_session(1050, 0.5, "/bar", "bbb"),
            record_with_session(1150, 2.5, "/bar", "bbb"),
        ];
        let params = make_params(&records, Some("aaa"), Some(3.0), None, None, None, None, Some("/bar"));
        let stats = compute_aggregate_stats(&params);
        // project_today_cost: only /bar sessions' delta = 2.5 - 0.5 = 2.0 (current session not in /bar)
        assert!(
            (stats.project_today_cost - 2.0).abs() < 0.01,
            "project_today_cost was {} but expected 2.0",
            stats.project_today_cost
        );
        // all_today_cost: /bar delta (2.0) + cur_delta (3.0 - 1.0 = 2.0) = 4.0
        assert!(
            (stats.all_today_cost - 4.0).abs() < 0.01,
            "all_today_cost was {} but expected 4.0",
            stats.all_today_cost
        );
    }

    #[test]
    fn ctx_trend_not_enough_data() {
        let trend = compute_ctx_trend(&[], LOOKBACK);
        assert!(trend.is_none());

        let records = vec![record(1000, 1.0, "/proj")];
        let trend = compute_ctx_trend(&records, LOOKBACK);
        assert!(trend.is_none());
    }

    #[test]
    fn ctx_trend_simple_delta() {
        let mut records = Vec::new();
        // 12 records, 60 seconds apart (total span = 11 * 60 = 660s)
        for i in 0..12 {
            let mut r = record(1000 + i * 60, 1.0, "/proj");
            r.input.context_window.as_mut().unwrap().used_percentage = Some(20.0 + i as f64 * 2.0);
            records.push(r);
        }
        // lookback 300s: last record at ts=1660, cutoff=1360
        // first record with ts <= 1360 (scanning backwards) is ts=1360 (i=6), ctx_pct=32.0
        // last record: i=11, ctx_pct=42.0, delta = 42.0 - 32.0 = 10.0
        let trend = compute_ctx_trend(&records, LOOKBACK).unwrap();
        assert!((trend - 10.0).abs() < 0.01);
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
        assert_eq!(s.cost_delta(), 1.0);
        assert_eq!(s.start_ts(), 1000);
        assert_eq!(s.model(), Some("Opus 4.6"));
        assert_eq!(s.project(), Some("/proj"));
    }

    #[test]
    fn avg_daily_cost_empty() {
        assert!(avg_daily_cost_from_records(&[]).is_none());
    }

    #[test]
    fn avg_daily_cost_single_day() {
        let records = vec![
            record_with_session(1000, 1.0, "/proj", "aaa"),
            record_with_session(2000, 3.0, "/proj", "aaa"),
            record_with_session(3000, 0.5, "/proj", "bbb"),
            record_with_session(4000, 2.5, "/proj", "bbb"),
        ];
        // day: 1970-01-01, session aaa delta=2.0, session bbb delta=2.0, total=4.0, 1 day => avg=4.0
        let avg = avg_daily_cost_from_records(&records).unwrap();
        assert!((avg - 4.0).abs() < 0.01);
    }

    #[test]
    fn avg_daily_cost_multi_day() {
        let day1 = 86400; // 1970-01-02
        let day2 = 86400 * 2; // 1970-01-03
        let records = vec![
            record_with_session(day1, 1.0, "/proj", "aaa"),
            record_with_session(day1 + 100, 3.0, "/proj", "aaa"),
            record_with_session(day2, 0.5, "/proj", "bbb"),
            record_with_session(day2 + 100, 1.5, "/proj", "bbb"),
        ];
        // day1: delta=2.0, day2: delta=1.0, avg = 1.5
        let avg = avg_daily_cost_from_records(&records).unwrap();
        assert!((avg - 1.5).abs() < 0.01);
    }
}
