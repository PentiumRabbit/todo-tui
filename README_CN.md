# todo-tui

键盘驱动的终端待办事项管理器，基于 Rust + [ratatui](https://github.com/ratatui-org/ratatui) 构建。

[English](README.md)

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

> **说明**
>
> 本项目由 AI 全程生成并维护。**不接受 Pull Request。** 如有问题或建议，请 [提交 Issue](../../issues)。

---

## 截图

![todo-tui](https://github.com/user-attachments/assets/4ecb1596-0387-4098-9d87-9d06da2338e5)

---

## 功能特性

- **过滤栏** — 按标签、状态（未完成/已完成/已取消）、今日到期、已逾期过滤
- **优先级** — 高/中/低，颜色区分
- **截止日期** — 过期/今日到期自动高亮
- **备注** — 每条 todo 支持备注说明，列表中内联显示
- **排序控制** — 按优先级/截止日/创建时间切换
- **完整增删改查** — 添加、编辑、删除、完成、取消
- **本地持久化** — SQLite 存储于 `~/.todo-tui/todos.db`
- **搜索** — 实时匹配标题、标签、备注
- **鼠标支持** — 点击与滚轮
- **语言切换** — `L` 键在中英文界面间切换
- **命令行快捷添加** — 在任意终端快速记录 todo，无需打开 TUI

---

## 安装

### Homebrew (macOS)

```bash
brew tap PentiumRabbit/tap
brew install todo-tui
```

### 直接下载

前往 [Releases](../../releases) 页面，下载对应平台的二进制文件。

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

**Ubuntu / Linux (x86_64) — `.deb` 包**
```bash
curl -LO https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui_0.4.0-1_amd64.deb
sudo dpkg -i todo-tui_0.4.0-1_amd64.deb
```

**Ubuntu / Linux (x86_64) — 二进制**
```bash
curl -L https://github.com/PentiumRabbit/todo-tui/releases/latest/download/todo-tui-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv todo-tui /usr/local/bin/
```

### 从源码构建

```bash
git clone https://github.com/PentiumRabbit/todo-tui.git
cd todo-tui
cargo build --release
./target/release/todo-tui
```

### 环境要求

- Rust 1.75+
- 终端尺寸 ≥ 80×24
- macOS 或 Linux

---

## 使用

```bash
todo-tui
```

首次启动自动在 `~/.todo-tui/todos.db` 创建数据库文件。

---

## 命令行用法

无需打开 TUI，直接在终端添加 todo：

```bash
todo-tui add <标题>
```

### 可选参数

| 参数 | 可选值 | 说明 |
|------|--------|------|
| `-p` | `high` \| `medium` \| `low` | 设置优先级（默认：`medium`） |
| `-t <标签>` | 任意字符串 | 添加标签（可多次使用） |
| `-d <时间>` | `'YYYY-MM-DD HH:MM'` | 设置截止日期 |

### 示例

```bash
# 添加一条简单的 todo
todo-tui add "买菜"

# 设置高优先级
todo-tui add "部署紧急修复" -p high

# 添加标签
todo-tui add "写单元测试" -t work -t rust

# 设置截止日期
todo-tui add "提交报告" -d '2026-05-20 17:00'

# 组合使用所有参数
todo-tui add "团队会议" -p high -t work -t meeting -d '2026-05-19 10:00'
```

### TUI 自动刷新

若 TUI 已在运行，通过 `todo-tui add` 添加的新条目将在约 500 ms 内自动显示，无需重启。

---

## 快捷键

### 普通模式

| 按键 | 动作 |
|------|------|
| `j` / `↓` | 下移 |
| `k` / `↑` | 上移 |
| `g` / `Home` | 跳到顶部 |
| `G` / `End` | 跳到底部 |
| `h` / `←` / `Tab` | 切换到过滤栏 |
| `Enter` | 打开详情 |
| `a` | 添加 |
| `e` | 编辑 |
| `d` | 删除 |
| `Space` | 切换完成状态 |
| `x` | 取消 |
| `s` | 切换排序方式 |
| `/` | 搜索 |
| `L` | 切换语言（中/英） |
| `?` | 帮助 |
| `q` | 退出 |

### 过滤栏

| 按键 | 动作 |
|------|------|
| `j` / `↓` | 下移 |
| `k` / `↑` | 上移 |
| `g` / `Home` | 跳到顶部 |
| `G` / `End` | 跳到底部 |
| `l` / `→` / `Tab` | 切换回列表 |
| `Enter` | 选择过滤条件 |

### 添加/编辑表单

| 按键 | 动作 |
|------|------|
| `Tab` | 下一字段 |
| `Shift+Tab` | 上一字段 |
| `↑` / `↓` | 切换优先级 |
| `Enter` | 提交 |
| `Esc` | 取消 |

---

## 项目结构

```
src/
├── main.rs          # 入口
├── app.rs           # 状态与事件
├── config.rs        # 配置持久化（~/.config/todo-tui/config.toml）
├── i18n.rs          # UI 文案（中/英）
├── models/          # 数据模型
├── storage/         # SQLite 持久化
└── ui/              # 渲染
    ├── theme.rs     # 颜色常量
    ├── list.rs      # 列表面板
    ├── detail.rs    # 详情弹窗
    ├── form.rs      # 表单
    ├── tags.rs      # 过滤栏
    └── help.rs      # 帮助弹窗
```

---

## 许可

MIT
