# claude-statusline

A fast, configurable status line for [Claude Code](https://claude.ai/code), written in Rust.

```
Opus 4.6  v2.1.63  ⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀  34%  $5.24  +245/-9  6h32m (api 10m)  …/apple/spiderweb
```

## Features

- Braille-dot context gauge with color-coded fill (green < 50%, yellow 50-79%, red 80%+)
- Model name, version, session cost, lines changed, duration, working directory
- Context exceeded warning when over the token limit
- Configurable via presets or custom element lists
- Near-instant startup (~2ms) thanks to compiled Rust binary

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

| Preset    | Elements                                                    |
|-----------|-------------------------------------------------------------|
| `minimal` | model, gauge, context                                       |
| `compact` | model, gauge, context, cost, cwd                            |
| `default` | model, gauge, context, duration, cwd, project, style         |
| `full`    | all elements                                                |

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

### Available elements

| Element              | Description                                |
|----------------------|--------------------------------------------|
| `model`              | Model display name (e.g. Opus 4.6)        |
| `version`            | Claude Code version                        |
| `gauge`              | Braille-dot context usage bar              |
| `context` / `ctx`    | Context usage percentage                   |
| `cost`               | Session cost in USD                        |
| `lines`              | Lines added/removed this session           |
| `duration` / `time`  | Session uptime and API wait time           |
| `cwd`                | Current working directory (shortened)      |
| `project` / `project_dir` | Project root directory (shortened)    |
| `style` / `output_style`  | Output style (hidden when "default")  |

## CLI usage

```
claude-statusline --help              # Full help
claude-statusline --list              # Show presets and elements
claude-statusline --preset compact    # Use a preset
claude-statusline --elements model,gauge,ctx,cost  # Custom elements
```

## How it works

Claude Code pipes a JSON object to stdin on each status line update. The binary parses it and renders the selected elements with ANSI color codes. The JSON includes fields like model info, context window usage, session cost, and workspace paths.
