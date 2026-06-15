# app 模块接口契约

**最近更新**: 2026-05-28（补 FEAT-todo-tui-001 + FEAT-todo-tui-002 变更）
**原始日期**: 2026-04-23

---

## AppState 结构

```rust
pub struct AppState {
    pub mode: AppMode,
    pub todos: Vec<Todo>,
    pub all_tags: Vec<String>,
    pub filter: FilterMode,
    pub tag_panel_index: usize,
    pub focus_tag_panel: bool,
    pub show_filter_panel: bool,
    pub selected_index: usize,
    pub sort_order: SortOrder,
    pub list_offset: usize,        // 滚动偏移
    pub search_query: String,
    pub regex_mode: bool,          // FEAT-002：正则搜索模式开关
    pub regex_error: Option<String>, // FEAT-002：当前正则编译错误，UI 层只读
    pub form: FormState,           // Add/Edit 表单状态
    pub error_message: Option<String>,
    pub config: Config,
    pub pending_reload: bool,      // FEAT-001：表单模式下延迟刷新标志
}
```

---

## 对外方法

### `AppState::new(storage: Storage, config: Config) -> Result<Self>`
- 输入：已初始化的 Storage 实例，Config
- 输出：加载了所有 todo 和 tag 的 AppState
- 异常：数据库读取失败时返回 Err

### `AppState::handle_event(&mut self, event: KeyEvent) -> Result<AppAction>`
- 输入：crossterm KeyEvent
- 输出：AppAction（Continue / Quit）
- 约束：同步调用，不阻塞

### `AppState::handle_mouse(&mut self, event: MouseEvent, layout_tag_panel: Rect, layout_list: Rect, form_areas: FormAreas) -> Result<()>`
- 输入：crossterm MouseEvent，UI 布局区域（由渲染层传入）
- 输出：Ok(()) 或 Err
- 约束：同步调用，不阻塞

### `AppState::trigger_reload(&mut self)`
- FEAT-001 新增
- Add/Edit 模式下设置 `pending_reload = true`；其他模式立即执行 `reload_from_db`
- 由 main.rs mtime 轮询检测到 DB 变化时调用

### `AppState::filtered_todos(&self) -> Vec<&Todo>`
- 输入：无（使用内部 `search_query`、`regex_mode`、`filter`、`sort_order`）
- 输出：过滤 + 排序后的 todo 引用列表
- 约束：只读（`&self`），不写入任何字段

### `AppState::selected_todo(&self) -> Option<&Todo>`
- 返回当前高亮条目的引用；列表为空时返回 None

### `AppState::tag_panel_items(&self) -> Vec<PanelItem>`
- 返回标签面板条目列表（内置项 + 用户标签）

---

## 私有辅助方法（不对 UI 层暴露）

| 方法 | 说明 |
|------|------|
| `reload_from_db(&mut self) -> Result<()>` | 从 SQLite 重新加载 todos（FEAT-001） |
| `update_regex_error(&mut self)` | 编译当前 search_query 为正则，写入 regex_error（FEAT-002） |

---

## AppMode 枚举

```rust
pub enum AppMode {
    Normal,
    Detail,        // 详情查看模式
    Add,
    Edit,
    DeleteConfirm,
    Search,
    Help,
}
```

## AppAction 枚举

```rust
pub enum AppAction {
    Continue,
    Quit,
}
```

## FilterMode 枚举

```rust
pub enum FilterMode {
    All,
    ByTag(String),
    ByStatus(TodoStatus),
    DueToday,
    Overdue,
}
```

## SortOrder 枚举

```rust
pub enum SortOrder {
    Default,      // Pending 优先 → 优先级 → 创建时间
    ByPriority,   // High → Medium → Low
    ByDueDate,    // 最近到期优先，无日期排末尾
    ByCreatedAt,  // 最新创建优先
}
```

## PanelItem 枚举

```rust
pub enum PanelItem {
    All,
    Status(TodoStatus),
    DueToday,
    Overdue,
    Tag(String),
}
```

---

## 约束

- UI 层（`src/ui/`）只读 AppState，不得写入任何字段（M-002）
- `filtered_todos` 是纯函数，不写 `regex_error`（写入由 `update_regex_error` 负责）
- `regex_error` 与 `error_message` 独立：`error_message` 每次 handle_event 开头被清空，无法持久；`regex_error` 跨事件保留
