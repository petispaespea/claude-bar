# Element calculations

How each element's displayed value is computed.

## Core elements (from stdin input)

| Element | Source | Displayed as |
|---|---|---|
| `model` | `input.model.display_name` | raw string, e.g. `Opus 4.6` |
| `version` | `input.version` | `v{version}` |
| `context` | `input.context_window.used_percentage` | bar + `{pct}%`, color: green <50%, yellow <80%, red >=80% |
| `tokens` | `input.context_window.{total_input_tokens, total_output_tokens}` | `{in}/{out}` with K/M suffixes |
| `cache` | `input.context_window.current_usage.{cache_read_input_tokens, cache_creation_input_tokens}` | `r:{read} w:{write}`, hidden when both zero |
| `cost` | `input.cost.total_cost_usd` | `${cost:.2}` |
| `lines` | `input.cost.{total_lines_added, total_lines_removed}` | `+{add}/-{del}`, hidden when both zero |
| `duration` | `input.cost.total_api_duration_ms` | formatted as `Xm`, `XhYm`, etc. |
| `wall_time` | `input.cost.total_duration_ms` | same format as duration |
| `git_branch` | resolved from `.git/HEAD` via `input.cwd` (pre-computed before rendering) | branch name string |
| `cwd` | `input.cwd` | shortened path (last 2 components) |
| `project` | `input.workspace.project_dir` | shortened path (last 2 components) |
| `style` | `input.output_style.name` | `[{name}]`, hidden when `"default"` |
| `cache_hit_rate` | derived from cache read/write tokens | `cache_read / (cache_read + cache_write) * 100`, displayed as `{pct}%` |

## Stats elements (require `[stats] enabled = true`)

All stats elements depend on today's stats file (`~/.local/share/claude-bar/stats/YYYY-MM-DD.jsonl`).

The `[stats] day_window` setting controls the time window:
- `"calendar"` (default) — uses today's UTC-dated file only
- `"rolling"` — uses a rolling 24-hour window (may include yesterday's records)

### Summary

| Element | Formula | Display | Condition |
|---|---|---|---|
| `project_daily_cost` | `Σ(final_cost - first_cost)` for completed sessions matching current project + `cur_delta` if current session matches project | `$X.XX/day` | — |
| `daily_budget` | `all_daily_cost / limit × 100` where `all_daily_cost = Σ(final_cost - first_cost)` all completed sessions + `cur_delta` | `$spent/$limit` + bar + `pct%` | `limit > 0` in config |
| `burn_rate` | `current_cost / (current_api_ms / 3,600,000)` | `$X.XX/hr` | `api_ms ≥ 60s` |
| `spend_rate` | `current_cost / (current_wall_ms / 3,600,000)` | `$X.XX/hr` | `wall_ms ≥ 300s` |
| `session_count` | number of sessions grouped by `(project, session_id)` matching current project | `#N` | — |
| `tok_per_dollar` | `current_out_tok / current_cost` | `Nk/$` | `cost > 0.001` |
| `cost_vs_avg` | `current_cost / (Σ other_sessions.final_cost / other_count)` | `X.X× avg` | `≥1 other session`, `avg > 0.001` |
| `ctx_trend` | `current_ctx_pct - ctx_pct[now - lookback_secs]` | `▲/▼/▸ ±N%` | `≥2 records` with `ctx_pct` |

### Key terms

- **`cur_delta`** = `max(0, current_cost - cur_session.first_cost)` — today-only spend for the live session
- **`first_cost`** / **`final_cost`** — first and last `total_cost_usd` recorded today for a session
- **delta-based** costs (`project_daily_cost`, `daily_budget`) use `final - first` to exclude pre-midnight spend
- **absolute** costs (`cost_vs_avg`) use `final_cost` to compare full session sizes

### Session grouping

Records are grouped into sessions by `(project_dir, session_id)`. All records sharing the same project and session ID belong to one session.

### Computed fields

#### `project_daily_cost`

Today's spend for the current project only.

```
project_delta = sum of (final_cost - first_cost) for completed sessions matching current_project
project_daily_cost = project_delta + (cur_delta if current session matches current_project, else 0)
```

Displayed as `$X.XX/day`.

#### `all_daily_cost`

Today's spend across all projects, delta-based.

```
all_delta = sum of (final_cost - first_cost) for all completed sessions
all_daily_cost = all_delta + cur_delta
```

Not directly rendered as its own element; used by `daily_budget`.

#### `burn_rate`

Cost per hour based on API duration. Only shown after 1 minute of API time.

```
burn_rate = current_cost / (current_api_ms / 3_600_000)
```

Requires: `current_api_ms >= 60_000`. Displayed as `$X.XX/hr`.

#### `spend_rate`

Cost per hour based on wall clock time. Only shown after 5 minutes.

```
spend_rate = current_cost / (current_wall_ms / 3_600_000)
```

Requires: `current_wall_ms >= 300_000`. Displayed as `$X.XX/hr`.

#### `session_count`

Number of sessions for the current project. Displayed as `#N`.

#### `daily_budget`

All-project daily spend vs configured limit. Uses `all_daily_cost`.

```
daily_budget_pct = all_daily_cost / limit * 100
```

Requires: `daily_budget.limit > 0` in config (default: `$100`). Displayed as `$spent/$limit` with optional bar and percentage. Color: green <50%, yellow <80%, red >=80%.

#### `tok_per_dollar`

Output tokens per dollar for the current session.

```
tok_per_dollar = current_out_tok / current_cost
```

Requires: `current_cost > 0.001`. Displayed as `{tokens}/$` with K/M suffixes.

#### `cost_vs_avg`

Current session cost relative to the average of other sessions today.

```
avg = sum(other_sessions.final_cost) / other_session_count
cost_vs_avg = current_cost / avg
```

Requires: at least 1 other session today with `avg > 0.001`. Displayed as `X.X× avg`.

Note: uses `final_cost()` (absolute cumulative cost), not deltas — this compares full session sizes, not today-only portions.

#### `ctx_trend`

Direction of context window usage over a configurable time window.

```
ctx_trend = current_ctx_pct - ctx_pct_from_lookback_secs_ago
```

Configurable via `[ctx_trend] lookback_secs` (default: `300` = 5 minutes).

Requires: at least 2 records today with `used_percentage` set. Displayed as:
- `▲ +N%` (red) when delta > 2
- `▼ -N%` (green) when delta < -2
- `▸ +N%` (dim) otherwise

## Alerts

Alerts are rule-based badges configured in `[[alert]]` TOML sections.

| Trigger | Condition | Default label |
|---|---|---|
| `ctx_exceeded` | `input.exceeds_200k_tokens == true` | `CTX over 200K` |
| `ctx_high` | `used_percentage >= threshold` | (user-defined) |
| `cost_high` | `daily_budget_pct >= 100` | `BUDGET exceeded` |

Severity controls badge color: `error` = red bg, `warn` = yellow bg, `info` = blue bg.
