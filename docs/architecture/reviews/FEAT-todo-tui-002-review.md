# FEAT-todo-tui-002 架构评审

> 架构师: arch-FEAT-todo-tui-002（dispatch#4）
> REQ: FEAT-todo-tui-002
> 日期: 2026-05-28
> 状态: 已确认

---

## 一、需求摘要

在已有的 `AppMode::Search` 搜索模式中，增加正则/普通模式切换（`Ctrl+R`）、正则匹配逻辑、以及非法正则内联错误提示。变更涉及 `AppState`（业务逻辑层）和状态栏渲染（UI 层），不涉及 storage/models 接口变更。

---

## 二、模块影响分析

| 模块/文件 | 变更类型 | 说明 |
|-----------|---------|------|
| `src/app.rs` | 修改 | 新增 `regex_mode`、`regex_error` 字段；扩展 `handle_search`；新增 `update_regex_error` 私有帮助函数；修改 `filtered_todos` 匹配分支 |
| `src/ui/mod.rs` | 修改 | 修改 `render_status_bar`，Search 模式下渲染 `[RE]` 指示符和错误消息 |
| `src/ui/theme.rs` | 修改 | 新增 `FG_REGEX_INDICATOR` 颜色常量 |
| `Cargo.toml` | 修改 | 新增 `regex = "1"` 依赖 |
| `src/storage/mod.rs` | 无变更 | 不涉及持久化 |
| `src/models/mod.rs` | 无变更 | 不涉及数据结构 |

**模块边界判断**：

- `app.rs` 的变更完全在业务逻辑层内，不越界
- `src/ui/` 只读取 `app.regex_mode` 和 `app.regex_error`（只读访问，符合"UI 只读约束" M-002）
- `src/ui/theme.rs` 新增颜色常量，不改已有常量，不越界
- `Cargo.toml` 新增外部 crate，属于独立改动，不与业务逻辑交叉

---

## 三、功能分层设计

| 功能点 | 落层 | 理由 |
|--------|------|------|
| `Ctrl+R` 键盘事件处理 | 业务逻辑层（app.rs） | 键盘事件路由在 `handle_search`，状态切换是业务逻辑 |
| `regex_mode` / `regex_error` 状态维护 | 业务逻辑层（app.rs） | 属于 `AppState` 状态机的一部分 |
| 正则编译与匹配 | 业务逻辑层（app.rs） | 是过滤规则的一部分，而非渲染职责 |
| `[RE]` 指示符和错误提示渲染 | UI 层（ui/mod.rs） | 纯渲染，读取 app 状态展示，不含判断逻辑 |
| 颜色常量定义 | UI 层（ui/theme.rs） | 符合颜色集中约束 M-003 |

---

## 四、状态管理设计

**新增字段**：

| 状态名 | 类型 | 归属 | 初始值 | 共享范围 | 说明 |
|--------|------|------|--------|---------|------|
| `regex_mode` | `bool` | `AppState`（app.rs） | `false` | AppState 内部，只读暴露给 UI | 当前是否处于正则搜索模式 |
| `regex_error` | `Option<String>` | `AppState`（app.rs） | `None` | AppState 内部，只读暴露给 UI | 当前正则编译错误消息；None 表示无错误 |

**不使用 `error_message` 字段的原因**：`error_message` 在每次 `handle_event` 开头被清空（`self.error_message = None`），无法在连续击键间持续持有正则错误状态。`regex_error` 需要跨事件持久，直至输入变为合法或退出搜索模式。

**状态重置规则**：

- 进入 Search 模式（`/` 键）：`search_query.clear()`，`regex_mode` 不变（按需求 FR-1 要求：每次退出后重置，再进入默认为 false，因此进入时应保持 false，但由于退出时已重置，进入时无需额外重置）
- 退出 Search 模式（`ESC`）：`search_query.clear(); regex_mode = false; regex_error = None;`
- 退出 Search 模式（`Enter`）：`regex_mode = false; regex_error = None;`（`search_query` 保留）

**状态通信**：单向只读——`AppState.regex_mode` 和 `AppState.regex_error` 由 `app.rs` 写，由 `src/ui/mod.rs` 读；符合项目单向数据流约束。

---

## 五、数据流设计

```text
用户击键（AppMode::Search）
  │
  ▼
handle_search(&mut self, event)
  ├─ Ctrl+R → toggle regex_mode → update_regex_error()
  ├─ Char(c) → push to search_query → update_regex_error()
  ├─ Backspace → pop search_query → update_regex_error()
  ├─ ESC → clear search_query; regex_mode=false; regex_error=None; mode=Normal
  └─ Enter → regex_mode=false; regex_error=None; mode=Normal
        │
        ▼
update_regex_error(&mut self)          [私有帮助函数]
  ├─ regex_mode=false → regex_error=None（直接返回）
  ├─ search_query 为空 → regex_error=None
  └─ 尝试编译正则
       ├─ 成功 → regex_error=None
       └─ 失败 → regex_error=Some(err.to_string())

渲染帧
  │
  ▼
filtered_todos(&self) → Vec<&Todo>
  ├─ regex_mode=false → 现有普通子串匹配逻辑（不变）
  ├─ regex_mode=true AND search_query 为空 → 返回全量
  ├─ regex_mode=true AND regex_error.is_some() → 返回空 Vec
  └─ regex_mode=true AND regex_error.is_none() → 编译正则 → 对 title/tags/notes 执行 is_match
        │
        ▼
render_status_bar(&self, ...) [只读访问 AppState]
  ├─ regex_mode=false → 现有显示逻辑（搜索词）
  ├─ regex_mode=true AND regex_error=None → "[RE] <search_query>"（FG_REGEX_INDICATOR 着色）
  └─ regex_mode=true AND regex_error=Some(msg) → "[RE:ERROR] <truncated_msg>"（ACTION_ERROR 着色）
```

**无 API 调用**：纯前端内存计算，无数据库写入。

---

## 六、接口契约

### 6.1 依赖方案（§一）

**推荐方案：引入 `regex = "1"` crate**

| 维度 | `regex` crate | 手写子串+通配符方案 |
|------|--------------|---------------------|
| 表达力 | 完整正则语法（`^$` `|` `(?i)` 捕获组等） | 仅能实现子集，与需求描述不符 |
| 大小写不敏感 | `RegexBuilder::case_insensitive(true)` 一行 | 需手动 `.to_lowercase()` 额外处理 |
| Unicode 支持 | 原生支持（Rust 默认 Unicode 模式） | 手写实现繁琐 |
| 错误报告 | `regex::Error` 有可读错误信息（直接满足 FR-3） | 手写没有统一错误类型 |
| 二进制体积增量 | ~1.5 MB（已在 Rust TUI 应用中被接受） | 零增量但功能不完整 |
| 维护成本 | 成熟 crate，API 稳定，社区维护 | 需长期自己维护解析器 |

**结论**：引入 `regex = "1"`，在 `Cargo.toml` `[dependencies]` 段添加该项。手写方案无法满足需求中"完整正则表达式"的要求，不予采用。

### 6.2 AppState 变更方案（§二）

**新增字段签名**（在 `AppState` 结构体，`src/app.rs`）：

```rust
pub regex_mode: bool,           // 初始值: false
pub regex_error: Option<String>, // 初始值: None
```

`AppState::new()` 初始化中追加：

```rust
regex_mode: false,
regex_error: None,
```

**退出 Search 模式重置逻辑**：

在 `handle_search` 函数内，`KeyCode::Esc` 分支：

```rust
KeyCode::Esc => {
    self.search_query.clear();
    self.regex_mode = false;
    self.regex_error = None;
    self.mode = AppMode::Normal;
    self.selected_index = 0;
}
```

在 `handle_search` 函数内，`KeyCode::Enter` 分支：

```rust
KeyCode::Enter => {
    self.regex_mode = false;
    self.regex_error = None;
    self.mode = AppMode::Normal;
    self.selected_index = 0;
}
```

### 6.3 Ctrl+R 键码处理（§三）

`KeyModifiers` 需加入 import：

```rust
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
```

在 `handle_search` 中新增 match arm（优先于通配符分支，置于 `KeyCode::Char(c)` 之前）：

```rust
KeyCode::Char('r') if event.modifiers.contains(KeyModifiers::CONTROL) => {
    self.regex_mode = !self.regex_mode;
    self.regex_error = None;
    if self.regex_mode {
        self.update_regex_error();
    }
    self.selected_index = 0;
}
```

**注意**：crossterm 0.28 在启用 `CONTROL` 修饰符时，`KeyCode::Char('r')` 中的字符为小写 `'r'`，不是 `'\x12'`（原始 ASCII 控制码）。

### 6.4 filtered_todos 改动点（§四）

`filtered_todos` 保持 `&self` 签名不变，仅修改 `search_ok` 计算分支：

```rust
let search_ok = if self.regex_mode {
    if self.search_query.is_empty() {
        true
    } else if self.regex_error.is_some() {
        false
    } else {
        // regex_error 为 None 时可以安全编译（update_regex_error 已验证合法）
        match regex::RegexBuilder::new(&self.search_query)
            .case_insensitive(true)
            .build()
        {
            Ok(re) => {
                re.is_match(&t.title)
                    || t.tags.iter().any(|tag| re.is_match(tag))
                    || t.notes.as_deref().map_or(false, |n| re.is_match(n))
            }
            Err(_) => false,
        }
    }
} else {
    // 原有普通子串逻辑，保持不变
    match &search {
        None => true,
        Some(q) => { /* 现有代码不变 */ ... }
    }
};
```

`filtered_todos` 中不写 `regex_error`（因为函数签名为 `&self`，无写权限）；`regex_error` 由 `update_regex_error` 在事件处理时维护。

**新增私有帮助函数** `update_regex_error`（`src/app.rs`）：

```rust
fn update_regex_error(&mut self) {
    if !self.regex_mode || self.search_query.is_empty() {
        self.regex_error = None;
        return;
    }
    match regex::RegexBuilder::new(&self.search_query)
        .case_insensitive(true)
        .build()
    {
        Ok(_) => self.regex_error = None,
        Err(e) => self.regex_error = Some(e.to_string()),
    }
}
```

调用时机：`Ctrl+R` 切换、`Char(c)` 输入、`Backspace` 删除。

### 6.5 UI 错误提示方案（§五）

**显示位置**：`src/ui/mod.rs` 中的 `render_status_bar` 函数，Search 模式分支。

**涉及颜色常量**（需在 `src/ui/theme.rs` 新增）：

```rust
// ── 正则搜索 ──────────────────────────────────────
pub const FG_REGEX_INDICATOR: Color = Color::Rgb(100, 200, 255); // [RE] 指示符，青蓝色
```

错误颜色复用已有的 `ACTION_ERROR: Color = Color::Rgb(220, 80, 80)`，无需新增。

**渲染逻辑修改**：将 `render_status_bar` 中的 Search 分支从返回 `&str` 改为直接渲染多色 `Line`：

```rust
AppMode::Search => {
    // 计算错误消息截断：最多占用状态栏宽度减去前缀后的剩余
    let prefix = if app.regex_mode { "[RE] " } else { "/" };
    let max_content = area.width.saturating_sub(prefix.len() as u16 + 2) as usize;

    let spans = if app.regex_mode {
        if let Some(err) = &app.regex_error {
            let truncated = truncate_str(err, max_content);
            vec![
                Span::styled("[RE:ERROR] ", Style::default().fg(theme::ACTION_ERROR)),
                Span::styled(truncated, Style::default().fg(theme::ACTION_ERROR)),
            ]
        } else {
            vec![
                Span::styled("[RE] ", Style::default().fg(theme::FG_REGEX_INDICATOR)),
                Span::raw(app.search_query.clone()),
            ]
        }
    } else {
        vec![
            Span::raw("/"),
            Span::raw(app.search_query.clone()),
        ]
    };
    // 直接渲染 Line，不走原有 hints &str 路径
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .style(Style::default().bg(theme::BG_STATUSBAR).fg(theme::FG_STATUSBAR)),
        area,
    );
    return; // 提前返回，跳过后续 hints 渲染
}
```

**错误消息截断策略**：使用 `unicode-width` 截断到 `area.width - prefix_width - 2` 字符宽度，末尾加 `…`，与 `list.rs` 现有的 `truncate_to_width` 函数语义相同（可提取为公共函数或直接内联）。

---

## 七、可复用组件识别

`list.rs` 中已有 `truncate_to_width` 函数用于按 Unicode 宽度截断字符串。状态栏错误消息截断需要同等功能。

**决策**：不提取为公共函数。理由：`truncate_to_width` 为 `list.rs` 私有函数，跨文件提取需将其移至公共工具模块，而截断逻辑简单（4-10 行），UI 层单次使用，内联实现无维护负担。过度提取违反 M-031 简洁原则。

---

## 八、方案对比

见 §6.1 依赖方案对比表。

---

## 九、任务拆分

| # | 描述 | 角色 | 涉及文件 | 依赖 | 可并行 |
|---|------|------|---------|------|--------|
| T1 | 新增 `regex = "1"` 依赖；`AppState` 新增 `regex_mode`/`regex_error` 字段；实现 `handle_search` 的 `Ctrl+R` 分支、`update_regex_error` 帮助函数、`filtered_todos` 正则分支；补充单元测试 | eng（app 模块） | `src/app.rs`、`Cargo.toml` | 无 | — |
| T2 | `theme.rs` 新增 `FG_REGEX_INDICATOR`；`render_status_bar` 修改，支持 `[RE]` 指示符和错误消息渲染 | eng（ui 模块） | `src/ui/mod.rs`、`src/ui/theme.rs` | T1（需要 app.rs 新字段编译通过） | 可在 T1 完成后立即开始 |
| T3 | 集成回归验证：验证普通搜索不受影响、正则切换、错误提示、退出重置 | te | — | T1 + T2 | 在 T1+T2 均完成后 |

---

## 十、模块列表

本次涉及以下模块（后续所有角色按此命名产出摘要文件）：

| 模块名称 | 模块描述 | 摘要文件（举例） |
|---------|---------|----------------|
| `app` | 应用状态机逻辑（AppState、事件处理、过滤） | `arch-app.md` / `eng-app.md` / `te-app.md` |
| `ui` | TUI 渲染（状态栏、列表、主题颜色） | `arch-ui.md` / `eng-ui.md` / `te-ui.md` |

**摘要命名一致性检查**：`.ai-team/summaries/` 中已存在以下文件，与本次模块列表名称检查：

| 已有文件 | 对应模块 | 状态 |
|---------|---------|------|
| `arch-app.md` | app | ✅ 一致 |
| `eng-app.md` | app | ✅ 一致 |
| `te-app.md` | app | ✅ 一致 |
| `eng-core.md` | core（非本次模块） | 不涉及，无需重命名 |
| `arch-core.md` | core（非本次模块） | 不涉及，无需重命名 |
| `te-core.md` | core（非本次模块） | 不涉及，无需重命名 |

`arch-ui.md` 不存在，本次由 arch 角色首次创建。

---

## 十一、回归影响分析

本次变更影响以下回归点（测试执行者回归时必须覆盖）：

| 回归点 | 受影响模块 | 回归优先级 |
|--------|----------|-----------|
| 普通搜索（子串匹配）功能不受影响 | `app`（filtered_todos） | P0 |
| 正则模式切换（Ctrl+R 奇偶次切换） | `app`（handle_search） | P0 |
| 正则匹配：title / tags / notes 均生效，大小写不敏感 | `app`（filtered_todos） | P0 |
| 非法正则时列表为空 + 状态栏出现错误提示 | `app` + `ui` | P0 |
| 合法正则修正后错误提示立即消失 | `app` + `ui` | P0 |
| 退出 Search 模式（ESC/Enter）后重进，模式重置为普通模式 | `app` | P1 |
| 搜索词为空时切换模式：全量显示，不崩溃 | `app` | P1 |
| 状态栏在隐藏模式下（show_statusbar=false）Search 模式仍强制显示（既有逻辑不变） | `ui` | P1 |
| 搜索行宽度极小时错误消息截断不破坏布局 | `ui` | P2 |
