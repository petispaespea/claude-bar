# claude-bar

A fast, configurable status line for [Claude Code](https://claude.ai/code), written in Rust.

<p align="center">
  <img src=".github/social-preview.svg" alt="claude-bar presets preview" width="800">
</p>

## Features

- Nerd Font icons with multiple icon sets (Octicons, Font Awesome)
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

Automatic:

```bash
claude-bar --setup
```

This adds the `statusLine` entry to `~/.claude/settings.json` pointing to the current binary. Restart Claude Code to apply.

Or manually add to `~/.claude/settings.json`:

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

### Icon sets

Icons are shown by default using Octicons. Switch sets with `--icon-set` or `CLAUDE_BAR_ICON_SET` env var:

| Set           | Flag                    | Env var                          |
|---------------|-------------------------|----------------------------------|
| Octicons      | `--icon-set octicons`   | `CLAUDE_BAR_ICON_SET=octicons`   |
| Font Awesome  | `--icon-set fa`         | `CLAUDE_BAR_ICON_SET=fa`         |
| None          | `--no-icons`            | `CLAUDE_BAR_ICONS=false`         |

### Available elements

| Element              | Description                           |
|----------------------|---------------------------------------|
| `model`              | Model display name (e.g. Opus 4.6)   |
| `version`            | Claude Code version                   |
| `gauge`              | Braille-dot context usage bar         |
| `context` / `ctx`    | Context usage percentage              |
| `tokens`             | Input/output token counts             |
| `cache`              | Cache read/write token counts         |
| `cost`               | Session cost in USD                   |
| `lines`              | Lines added/removed this session      |
| `duration` / `time`  | API wait time                         |
| `cwd`                | Current working directory (shortened) |
| `project` / `project_dir` | Project root directory (shortened)   |
| `style` / `output_style`  | Output style (hidden when "default") |

## CLI usage

```
claude-bar --setup                        # Auto-configure ~/.claude/settings.json
claude-bar --help                         # Full help
claude-bar --list                         # Show presets, elements, and icon sets
claude-bar --demo                         # Render with sample data (no stdin needed)
claude-bar --demo --preset full           # Preview a preset
claude-bar --demo --no-icons              # Preview without icons
claude-bar --demo --icon-set fa           # Preview with Font Awesome icons
claude-bar --demo --elements model,gauge,ctx  # Preview custom layout
```

## How it works

Claude Code pipes a JSON object to stdin on each status line update. The binary parses it and renders the selected elements with ANSI color codes. The JSON includes fields like model info, context window usage, token counts, cache stats, session cost, and workspace paths.
