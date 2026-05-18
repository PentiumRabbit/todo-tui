# todo-tui

A keyboard-driven terminal todo manager built with Rust and [ratatui](https://github.com/ratatui-org/ratatui).

[中文](README_CN.md)

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## Screenshot

![todo-tui](https://github.com/user-attachments/assets/4ecb1596-0387-4098-9d87-9d06da2338e5)

---

## Features

- **Filter panel** — filter by tag, status (Pending/Done/Cancelled), due today, or overdue
- **Priority levels** — High / Medium / Low with color indicators
- **Due dates** — precise to the minute, overdue and today's tasks highlighted
- **Notes** — optional multi-line notes field displayed inline in the list
- **Sort control** — cycle sort by priority / due date / created time
- **Full CRUD** — add, edit, delete, complete, cancel
- **Persistent storage** — SQLite at `~/.config/todo-tui/todos.db`
- **Search** — matches title, tags, and notes in real time
- **Mouse support** — click and scroll in addition to keyboard
- **Language toggle** — switch between English and Chinese with `L`
- **Quick add from CLI** — add todos from any terminal without opening the TUI

---

## Installation

### Homebrew (macOS)

```bash
brew tap PentiumRabbit/tap
brew install todo-tui
```

### cargo install

```bash
cargo install todo-tui
```

Or install directly from source:

```bash
cargo install --git https://github.com/PentiumRabbit/todo-tui.git
```

### Download from Release

Go to the [Releases](../../releases) page and download the binary for your platform.

**macOS (Apple Silicon)**
```bash
curl -L https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui-aarch64-apple-darwin.tar.gz | tar xz
sudo mv todo-tui /usr/local/bin/
```

**macOS (Intel)**
```bash
curl -L https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui-x86_64-apple-darwin.tar.gz | tar xz
sudo mv todo-tui /usr/local/bin/
```

**Ubuntu / Linux (x86_64) — `.deb` package**
```bash
curl -LO https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui_0.4.0-1_amd64.deb
sudo dpkg -i todo-tui_0.4.0-1_amd64.deb
```

**Ubuntu / Linux (x86_64) — binary**
```bash
curl -L https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv todo-tui /usr/local/bin/
```

### Build from source

```bash
git clone https://github.com/PentiumRabbit/todo-tui.git
cd todo-tui
cargo build --release
./target/release/todo-tui
```

### Requirements

- Rust 1.75+
- Terminal size ≥ 80×24
- macOS or Linux

---

## Usage

```bash
todo-tui
```

Data and config are created automatically on first launch at `~/.config/todo-tui/`.

---

## CLI Usage

Add todos directly from the command line without launching the TUI:

```bash
todo-tui add <title>
```

### Options

| Flag | Values | Description |
|------|--------|-------------|
| `-p` | `high` \| `medium` \| `low` | Set priority (default: `medium`) |
| `-t <tag>` | any string | Add a tag (repeatable) |
| `-d <datetime>` | `'YYYY-MM-DD HH:MM'` | Set due date |

### Examples

```bash
# Add a simple todo
todo-tui add "Buy groceries"

# Add with high priority
todo-tui add "Deploy hotfix" -p high

# Add with tags
todo-tui add "Write tests" -t work -t rust

# Add with due date
todo-tui add "Submit report" -d '2026-05-20 17:00'

# Combine all options
todo-tui add "Team meeting" -p high -t work -t meeting -d '2026-05-19 10:00'
```

### TUI Auto-refresh

If the TUI is already open, new items added via `todo-tui add` will appear automatically within approximately 500 ms — no restart needed.

---

## Keyboard Shortcuts

### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down (wraps to top) |
| `k` / `↑` | Move up (wraps to bottom) |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |
| `h` / `←` / `Tab` | Focus filter panel |
| `Enter` | Open detail |
| `a` | Add todo |
| `e` | Edit todo |
| `d` | Delete todo |
| `Space` | Toggle complete |
| `x` | Cancel todo |
| `s` | Cycle sort order |
| `/` | Search |
| `H` | Toggle filter panel |
| `B` | Toggle status bar |
| `L` | Toggle language (En/Zh) |
| `?` | Help |
| `q` | Quit |

### Filter Panel

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down (wraps) |
| `k` / `↑` | Move up (wraps) |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |
| `l` / `→` / `Tab` | Focus todo list |
| `Enter` | Select filter |
| `D` | Delete selected tag |

### Add / Edit Form

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Prev field |
| `↑` / `↓` | Cycle priority (on Priority field) |
| `←` / `→` | Move tag cursor (on Tags field) |
| `Backspace` | Delete char / select & delete tag |
| `←` / `→` | Switch date segment (on Due Date field) |
| `↑` / `↓` / scroll | Adjust date segment value |
| `c` | Clear due date (on Due Date field) |
| `Enter` | Submit |
| `Esc` | Cancel |

---

## Data & Config

| Path | Description |
|------|-------------|
| `~/.config/todo-tui/todos.db` | SQLite database |
| `~/.config/todo-tui/config.toml` | Language and UI preferences |

---

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # State & event handling
├── config.rs        # Config persistence
├── i18n.rs          # UI strings (En/Zh)
├── models/          # Data models
├── storage/         # SQLite persistence
└── ui/              # Rendering
    ├── theme.rs     # Color constants
    ├── list.rs      # Todo list panel
    ├── detail.rs    # Detail popup
    ├── form.rs      # Add/edit form
    ├── tags.rs      # Filter panel
    └── help.rs      # Help overlay
```

---

## License

MIT
