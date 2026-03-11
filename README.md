# claude-statusline

A fast, configurable status line for [Claude Code](https://claude.ai/code), written in Rust.

```
󱚣 Opus 4.6  󰹰 ⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀  󰁫 34%   10m   …/my-project   …/my-project
```

## Features

- Nerd Font icons for each element (optional, on by default)
- Braille-dot context gauge with color-coded fill (green < 50%, yellow 50-79%, red 80%+)
- Model name, version, session cost, lines changed, API wait time, working directory
- Context exceeded warning when over the token limit
- Configurable via presets or custom element lists
- Near-instant startup (~3ms) thanks to compiled Rust binary

## Installation

```bash
git clone <repo-url>
cd claude-statusline
cargo build --release
```

The binary is at `target/release/claude-statusline`.

## Setup

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "/path/to/claude-statusline"
  }
}
```

## Configuration

Configuration priority: CLI flags > `CLAUDE_STATUSLINE` env var > `default` preset.

### Presets

Set via `--preset` flag or `CLAUDE_STATUSLINE` env var:

| Preset    | Elements                                              |
|-----------|-------------------------------------------------------|
| `minimal` | model, gauge, context                                 |
| `compact` | model, gauge, context, cost, cwd                      |
| `default` | model, gauge, context, duration, cwd, project, style  |
| `full`    | all elements                                          |

```json
{
  "env": {
    "CLAUDE_STATUSLINE": "compact"
  }
}
```

### Custom layout

Pick individual elements with `--elements` or a comma-separated env var:

```json
{
  "env": {
    "CLAUDE_STATUSLINE": "model,gauge,ctx,cost,cwd"
  }
}
```

### Icons

Nerd Font icons are shown by default. Disable with:

- `--no-icons` flag
- `"CLAUDE_STATUSLINE_ICONS": "false"` in the settings.json `env` block

### Available elements

| Element              | Icon | Description                           |
|----------------------|------|---------------------------------------|
| `model`              | 󱚣    | Model display name (e.g. Opus 4.6)   |
| `version`            |     | Claude Code version                   |
| `gauge`              | 󰹰    | Braille-dot context usage bar         |
| `context` / `ctx`    | 󰈁    | Context usage percentage              |
| `cost`               |     | Session cost in USD                   |
| `lines`              | 󰷈    | Lines added/removed this session      |
| `duration` / `time`  |     | API wait time                         |
| `cwd`                |     | Current working directory (shortened) |
| `project` / `project_dir` |  | Project root directory (shortened)   |
| `style` / `output_style`  |  | Output style (hidden when "default") |

## CLI usage

```
claude-statusline --help              # Full help
claude-statusline --list              # Show presets and elements
claude-statusline --demo              # Render with sample data (no stdin needed)
claude-statusline --demo --preset full        # Preview a preset
claude-statusline --demo --no-icons           # Preview without icons
claude-statusline --demo --elements model,gauge,ctx  # Preview custom layout
claude-statusline --preset compact    # Use a preset (reads stdin)
claude-statusline --elements model,gauge,ctx,cost     # Custom elements (reads stdin)
```

## How it works

Claude Code pipes a JSON object to stdin on each status line update. The binary parses it and renders the selected elements with ANSI color codes. The JSON includes fields like model info, context window usage, session cost, and workspace paths.
