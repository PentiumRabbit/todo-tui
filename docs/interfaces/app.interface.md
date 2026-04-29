# app 模块接口契约

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 3/6

---

## AppState 结构

```rust
pub struct AppState {
    pub mode: AppMode,
    pub todos: Vec<Todo>,
    pub categories: Vec<Category>,
    pub selected_index: usize,
    pub list_offset: usize,        // 滚动偏移
    pub search_query: String,
    pub form: FormState,           // 添加/编辑表单状态
    pub error_message: Option<String>,
}
```

---

## 对外方法

### `AppState::new(storage: Storage) -> Result<AppState>`
- 输入：已初始化的 Storage 实例
- 输出：加载了所有 todo 和 category 的 AppState
- 异常：数据库读取失败时返回 Err

### `AppState::handle_event(&mut self, event: KeyEvent) -> Result<AppAction>`
- 输入：crossterm KeyEvent
- 输出：AppAction（Continue / Quit）
- 约束：同步调用，不阻塞

### `AppState::filtered_todos(&self) -> Vec<&Todo>`
- 输入：无（使用内部 search_query）
- 输出：过滤后的 todo 引用列表
- 约束：只读，纯函数

---

## AppMode 枚举

```rust
pub enum AppMode {
    Normal,
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
