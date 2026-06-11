# Todo TUI

A terminal-based todo list application built with Rust and Ratatui.

## Installation

### Cargo Install

bash
cargo install --git https://github.com/yourusername/todo-tui


### Source Build

bash
git clone https://github.com/yourusername/todo-tui.git
cd todo-tui
cargo build --release
# Binary at ./target/release/todo-tui


## Quick Start

bash
# Run the application
todo-tui

# Or if built from source
./target/release/todo-tui


### Basic Usage

- `Tab` / `Shift+Tab` — Navigate between panels
- `Enter` — Select / confirm
- `n` — Create new todo
- `e` — Edit selected todo
- `d` — Delete selected todo
- `q` / `Esc` — Quit / go back
- `?` — Show help

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TODO_TUI_DATA_DIR` | `~/.todo-tui` | Directory for data storage |
| `TODO_TUI_THEME` | `default` | UI theme (default, dark, light) |

### Configuration File

Create `~/.todo-tui/config.toml`:

toml
[general]
data_dir = "~/.todo-tui"
theme = "default"

[editor]
default_priority = "medium"
default_category = "general"

[display]
show_completed = true
items_per_page = 20


## Project Structure


todo-tui/
├── Cargo.toml          # Project manifest and dependencies
├── src/
│   ├── main.rs         # Entry point
│   ├── app.rs          # Application state and event loop
│   ├── ui/             # UI rendering components
│   │   ├── mod.rs
│   │   ├── list.rs     # Todo list panel
│   │   ├── detail.rs   # Todo detail panel
│   │   └── help.rs     # Help overlay
│   ├── models/         # Data models
│   │   ├── mod.rs
│   │   └── todo.rs     # Todo item struct
│   ├── storage/        # Data persistence
│   │   ├── mod.rs
│   │   └── file.rs     # File-based storage
│   └── config/         # Configuration handling
│       ├── mod.rs
│       └── settings.rs # Config parsing
└── docs/               # Documentation
    └── api.md          # API documentation


## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [ratatui](https://crates.io/crates/ratatui) | 0.26+ | Terminal UI framework |
| [crossterm](https://crates.io/crates/crossterm) | 0.27+ | Terminal manipulation |
| [serde](https://crates.io/crates/serde) | 1.0+ | Serialization |
| [serde_json](https://crates.io/crates/serde_json) | 1.0+ | JSON data format |
| [toml](https://crates.io/crates/toml) | 0.8+ | Config file parsing |
| [chrono](https://crates.io/crates/chrono) | 0.4+ | Date/time handling |
| [dirs](https://crates.io/crates/dirs) | 5.0+ | System directory paths |

## License

MIT
