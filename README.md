# todo-tui

A keyboard-driven terminal todo manager built with Rust and [ratatui](https://github.com/ratatui-org/ratatui).

键盘驱动的终端待办事项管理器，基于 Rust + ratatui 构建。

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

> **Note / 说明**
>
> This project is entirely generated and maintained by AI. **Pull Requests are not accepted.** If you find a bug or have a suggestion, please [open an Issue](../../issues).
>
> 本项目由 AI 全程生成并维护。**不接受 Pull Request。** 如有问题或建议，请 [提交 Issue](../../issues)。

---

## Features / 功能特性

- **Tag filtering** — sidebar tag panel with instant filter / **标签过滤** — 侧边栏标签面板，实时过滤
- **Priority levels** — High / Medium / Low with color indicators / **优先级** — 高/中/低，颜色区分
- **Due dates** — overdue and today's tasks highlighted / **截止日期** — 过期/今日到期自动高亮
- **Full CRUD** — add, edit, delete, complete, cancel / **完整增删改查** — 添加、编辑、删除、完成、取消
- **Persistent storage** — SQLite at `~/.todo-tui/todos.db` / **本地持久化** — SQLite 存储
- **Search** — real-time keyword filter / **搜索** — 实时关键词过滤
- **Mouse support** — click and scroll in addition to keyboard / **鼠标支持** — 点击与滚轮

---

## Installation / 安装

### Build from source / 从源码构建

```bash
git clone https://github.com/PentiumRabbit/todo-tui.git
cd todo-tui
cargo build --release
./target/release/todo-tui
```

### Requirements / 环境要求

- Rust 1.75+
- Terminal size ≥ 80×24 / 终端尺寸 ≥ 80×24
- macOS or Linux / macOS 或 Linux

---

## Usage / 使用

```bash
todo-tui
```

The data file is created automatically at `~/.todo-tui/todos.db` on first launch.

首次启动自动在 `~/.todo-tui/todos.db` 创建数据库文件。

---

## Keyboard Shortcuts / 快捷键

### Normal Mode / 普通模式

| Key / 按键 | Action / 动作 |
|-----------|--------------|
| `j` / `↓` | Move down / 下移 |
| `k` / `↑` | Move up / 上移 |
| `g` / `Home` | Jump to top / 跳到顶部 |
| `G` / `End` | Jump to bottom / 跳到底部 |
| `←` / `Tab` | Focus tag panel / 切换到标签面板 |
| `Enter` | Open detail / 打开详情 |
| `a` | Add todo / 添加 |
| `e` | Edit todo / 编辑 |
| `d` | Delete todo / 删除 |
| `Space` | Toggle complete / 切换完成状态 |
| `x` | Cancel todo / 取消 |
| `/` | Search / 搜索 |
| `?` | Help / 帮助 |
| `q` | Quit / 退出 |

### Tag Panel / 标签面板

| Key / 按键 | Action / 动作 |
|-----------|--------------|
| `j` / `↓` | Move down / 下移 |
| `k` / `↑` | Move up / 上移 |
| `→` / `Tab` | Focus todo list / 切换回列表 |
| `Enter` | Select tag filter / 选择标签过滤 |

### Add / Edit Form / 添加编辑表单

| Key / 按键 | Action / 动作 |
|-----------|--------------|
| `Tab` | Next field / 下一字段 |
| `Shift+Tab` | Prev field / 上一字段 |
| `↑` / `↓` | Cycle priority / 切换优先级 |
| `Enter` | Submit / 提交 |
| `Esc` | Cancel / 取消 |

---

## Project Structure / 项目结构

```
src/
├── main.rs          # Entry point / 入口
├── app.rs           # State & event handling / 状态与事件
├── models/          # Data models / 数据模型
├── storage/         # SQLite persistence / 持久化
└── ui/              # Rendering / 渲染
    ├── theme.rs     # Color constants / 颜色常量
    ├── list.rs      # Todo list panel / 列表面板
    ├── detail.rs    # Detail popup / 详情弹窗
    ├── form.rs      # Add/edit form / 表单
    ├── tags.rs      # Tag panel / 标签面板
    └── help.rs      # Help overlay / 帮助弹窗
```

---

## License / 许可

MIT
