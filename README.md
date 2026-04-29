# todo-tui

A keyboard-driven terminal todo manager built with Rust and [ratatui](https://github.com/ratatui-org/ratatui).

[中文](README_CN.md)

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

> **Note**
>
> This project is entirely generated and maintained by AI. **Pull Requests are not accepted.** If you find a bug or have a suggestion, please [open an Issue](../../issues).

---

## Screenshot

![todo-tui](https://github.com/user-attachments/assets/568ccc66-e6cd-402a-a32d-49f37abb404f)

---

## Features

- **Filter panel** — filter by tag, status (Pending/Done/Cancelled), due today, or overdue
- **Priority levels** — High / Medium / Low with color indicators
- **Due dates** — overdue and today's tasks highlighted
- **Notes** — optional notes field displayed inline in the list
- **Sort control** — cycle sort by priority / due date / created time
- **Full CRUD** — add, edit, delete, complete, cancel
- **Persistent storage** — SQLite at `~/.todo-tui/todos.db`
- **Search** — matches title, tags, and notes in real time
- **Mouse support** — click and scroll in addition to keyboard
- **Language toggle** — switch between English and Chinese with `L`

---

## Installation

### Homebrew (macOS)

```bash
brew tap PentiumRabbit/tap
brew install todo-tui
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
curl -LO https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui_0.2.0-1_amd64.deb
sudo dpkg -i todo-tui_0.2.0-1_amd64.deb
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

The data file is created automatically at `~/.todo-tui/todos.db` on first launch.

---

## Keyboard Shortcuts

### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
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
| `L` | Toggle language (En/Zh) |
| `?` | Help |
| `q` | Quit |

### Filter Panel

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |
| `l` / `→` / `Tab` | Focus todo list |
| `Enter` | Select filter |

### Add / Edit Form

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Prev field |
| `↑` / `↓` | Cycle priority |
| `Enter` | Submit |
| `Esc` | Cancel |

---

## Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # State & event handling
├── config.rs        # Config persistence (~/.config/todo-tui/config.toml)
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
