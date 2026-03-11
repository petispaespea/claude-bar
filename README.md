# claude-bar

A fast, configurable status line for [Claude Code](https://claude.ai/code), written in Rust.

<p align="center">
  <img src=".github/social-preview.svg" alt="claude-bar presets preview" width="800">
</p>

## Features

- Nerd Font icons for each element (optional, on by default)
- Braille-dot context gauge with color-coded fill (green < 50%, yellow 50-79%, red 80%+)
- Model name, version, session cost, token counts, lines changed, API wait time, working directory
- Cache statistics (read/write token counts)
- Context exceeded warning when over the token limit
- Configurable via presets or custom element lists
- Near-instant startup (~3ms) thanks to compiled Rust binary

## Installation

```bash
git clone <repo-url>
cd claude-bar
cargo build --release
```

The binary is at `target/release/claude-bar`.

## Setup

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "/path/to/claude-bar"
  }
}
```

## Configuration

Configuration priority: CLI flags > `CLAUDE_BAR` env var > `default` preset.

### Presets

Set via `--preset` flag or `CLAUDE_BAR` env var:

| Preset    | Elements                                                      |
|-----------|---------------------------------------------------------------|
| `minimal` | model, gauge, context                                         |
| `compact` | model, gauge, context, cost, cwd                              |
| `default` | model, gauge, context, tokens, duration, cwd, project, style  |
| `full`    | all elements                                                  |

```json
{
  "env": {
    "CLAUDE_BAR": "compact"
  }
}
```

### Custom layout

Pick individual elements with `--elements` or a comma-separated env var:

```json
{
  "env": {
    "CLAUDE_BAR": "model,gauge,ctx,cost,cwd"
  }
}
```

### Icons

Nerd Font icons are shown by default. Disable with:

- `--no-icons` flag
- `"CLAUDE_BAR_ICONS": "false"` in the settings.json `env` block

### Available elements

| Element              | Icon | Description                           |
|----------------------|------|---------------------------------------|
| `model`              | 󱚣    | Model display name (e.g. Opus 4.6)   |
| `version`            |     | Claude Code version                   |
| `gauge`              | 󰹰    | Braille-dot context usage bar         |
| `context` / `ctx`    | 󰈁    | Context usage percentage              |
| `tokens`             | 󰒠    | Input/output token counts             |
| `cache`              |     | Cache read/write token counts         |
| `cost`               |     | Session cost in USD                   |
| `lines`              | 󰷈    | Lines added/removed this session      |
| `duration` / `time`  |     | API wait time                         |
| `cwd`                |     | Current working directory (shortened) |
| `project` / `project_dir` |  | Project root directory (shortened)   |
| `style` / `output_style`  |  | Output style (hidden when "default") |

## CLI usage

```
claude-bar --help              # Full help
claude-bar --list              # Show presets and elements
claude-bar --demo              # Render with sample data (no stdin needed)
claude-bar --demo --preset full        # Preview a preset
claude-bar --demo --no-icons           # Preview without icons
claude-bar --demo --elements model,gauge,ctx  # Preview custom layout
claude-bar --preset compact    # Use a preset (reads stdin)
claude-bar --elements model,gauge,ctx,cost     # Custom elements (reads stdin)
```

## How it works

Claude Code pipes a JSON object to stdin on each status line update. The binary parses it and renders the selected elements with ANSI color codes. The JSON includes fields like model info, context window usage, session cost, and workspace paths.
