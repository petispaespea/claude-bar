# claude-bar

A configurable status line for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Shows model info, context usage, token counts, cost, and more — rendered with [Nerd Font](https://www.nerdfonts.com/) icons.

<p align="center">
  <img src=".github/social-preview.svg" alt="claude-bar presets preview" width="800">
</p>

## Quick start

```bash
brew tap petispaespea/tap
brew install claude-bar
claude-bar --setup
```

Or build from source:

```bash
git clone https://github.com/petispaespea/claude-bar.git
cd claude-bar
cargo build --release
./target/release/claude-bar --setup
```

The `--setup` flag adds the `statusLine` entry to `~/.claude/settings.json`. Restart Claude Code to see it.

Preview without running Claude Code:

```bash
claude-bar --demo
claude-bar --demo --preset full
```

## Configuration

All settings can be passed as CLI flags or set via env vars in `~/.claude/settings.json`. CLI flags take priority over env vars, which take priority over the TOML config, which falls back to the `default` preset.

### Presets

| Preset    | Elements                                                        |
|-----------|-----------------------------------------------------------------|
| `minimal` | model, context, alert                                           |
| `compact` | model, context, cost, cwd, alert                                |
| `default` | model, context, tokens, duration, cwd, project, style, alert    |
| `full`    | all elements (3 lines)                                          |

Set via `--preset` flag or `CLAUDE_BAR` env var:

```json
{
  "env": {
    "CLAUDE_BAR": "compact"
  }
}
```

### Custom layout

Cherry-pick elements with `--elements` or a comma-separated `CLAUDE_BAR` value:

```json
{
  "env": {
    "CLAUDE_BAR": "model,context,cost,cwd"
  }
}
```

### Multi-line layout

Use `---` as a separator to split into multiple lines:

```json
{
  "env": {
    "CLAUDE_BAR": "model,context,tokens,---,cost,duration,wall_time"
  }
}
```

Or in TOML:

```toml
[layout]
elements = ["model", "context", "tokens", "---", "cost", "duration", "wall_time"]
```

### Icon sets

| Set           | Flag / env var                                          |
|---------------|---------------------------------------------------------|
| Octicons      | default, or `--icon-set octicons`                       |
| Font Awesome  | `--icon-set fa` or `CLAUDE_BAR_ICON_SET=fa`             |
| None          | `--no-icons` or `CLAUDE_BAR_ICON_SET=none`              |

## TOML configuration

Configuration can also be supplied via a TOML file. The path resolution follows this precedence:

1. `--config <path>` CLI flag
2. `CLAUDE_BAR_CONFIG` environment variable
3. `$XDG_CONFIG_HOME/claude-bar.toml` (if set)
4. `~/.config/claude-bar.toml` (fallback)

To generate a starter config:

```bash
claude-bar --print-config > ~/.config/claude-bar.toml
```

Example TOML:

```toml
separator = "  "

[layout]
elements = ["model", "context", "tokens", "cwd"]

[model]
symbol = " "
style = "cyan"
```

Per-element fields:

- `symbol` (string): icon or text prefix
- `style` (string): space-separated style names

The `context` element also supports `bar_style` (`braille`, `block`, `shade`, `ascii`), `width`, `show_bar`, and `show_pct`.

Style vocabulary: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `bold`, `dim`, `italic`, `underline`. Example: `style = "bold red"`.

The `context` and `lines` elements default to empty style and use dynamic color based on runtime values.

## Elements

### Core elements

| Name                       | Description                                |
|----------------------------|--------------------------------------------|
| `model`                    | Model display name (e.g. Opus 4.6)         |
| `version`                  | Claude Code version                        |
| `context`, `ctx`           | Context bar + percentage (color-coded)     |
| `tokens`                   | Input/output token counts                  |
| `cache`                    | Cache read/write tokens                    |
| `cost`                     | Session cost in USD                        |
| `lines`                    | Lines added/removed                        |
| `duration`, `time`         | API wait time                              |
| `wall_time`, `wall`, `elapsed` | Wall clock elapsed time                |
| `cwd`                      | Working directory (shortened)              |
| `project`, `project_dir`   | Project root (shortened)                   |
| `style`, `output_style`    | Output style (hidden when "default")       |
| `alert`                    | Conditional badges (context, budget)       |

### Input-only elements

| Name                       | Description                                |
|----------------------------|--------------------------------------------|
| `cache_hit_rate`, `cache_hit` | Cache hit percentage                    |

### Stats elements

These require `[stats] enabled = true` in the TOML config:

| Name            | Description                              |
|-----------------|------------------------------------------|
| `project_daily_cost` | Today's spend for current project (alias: `daily_cost`) |
| `burn_rate`     | Cost per hour (API duration)             |
| `spend_rate`    | Cost per hour (wall clock)               |
| `session_count` | Number of sessions today                 |
| `daily_budget`  | Daily spend limit with progress bar      |
| `tok_per_dollar`| Output tokens per dollar                 |
| `cost_vs_avg`   | Current session cost vs historical avg   |
| `ctx_trend`     | Context usage direction over last 10 renders |

## CLI reference

```
claude-bar --setup                            # Configure ~/.claude/settings.json
claude-bar --print-config                     # Generate TOML config to stdout
claude-bar --config <path> --demo             # Use custom config file
claude-bar --demo                             # Preview with sample data
claude-bar --demo --preset full               # Preview a specific preset
claude-bar --demo --elements model,context,cost  # Preview a custom layout
claude-bar --demo --icon-set fa               # Preview Font Awesome icons
claude-bar --demo --no-icons                  # Preview without icons
claude-bar --info                             # Show all presets, elements, icon sets
claude-bar --stats                            # Show usage statistics
claude-bar --stats --stats-days 30            # Stats for last 30 days
claude-bar --stats --stats-project ~/myproj   # Stats for a specific project
claude-bar --stats-clear --yes                # Delete the stats log
claude-bar --completions bash                 # Generate shell completions
claude-bar --help                             # Full usage info
```
