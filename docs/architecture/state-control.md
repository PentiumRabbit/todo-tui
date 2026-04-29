# todo-tui 状态控制设计

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 6/6

---

## 状态空间定义

### AppMode（应用模式）

```rust
pub enum AppMode {
    Normal,          // 列表浏览，可导航和操作
    Add,             // 添加 todo 弹窗
    Edit,            // 编辑 todo 弹窗
    DeleteConfirm,   // 删除确认弹窗
    Search,          // 搜索模式
    Help,            // 帮助弹窗（叠加在 Normal 上）
}
```

### 状态转换规则

```
Normal ──[a]──▶ Add ──[Enter]──▶ Normal
                    ──[Esc]───▶ Normal

Normal ──[e]──▶ Edit ──[Enter]──▶ Normal
                     ──[Esc]───▶ Normal

Normal ──[d]──▶ DeleteConfirm ──[y/Enter]──▶ Normal
                               ──[n/Esc]───▶ Normal

Normal ──[/]──▶ Search ──[Enter/Esc]──▶ Normal

Normal ──[?]──▶ Help ──[?/Esc]──▶ Normal

任意模式 ──[q]──▶ 退出（仅 Normal 模式响应 q）
```

---

## FormState（表单状态）

```rust
pub struct FormState {
    pub title: String,
    pub category_id: Option<i64>,
    pub priority: Priority,
    pub due_date: String,         // 用户输入的原始字符串
    pub focused_field: FormField,
    pub editing_todo_id: Option<i64>,  // None = Add, Some = Edit
    pub title_error: Option<String>,
    pub due_date_error: Option<String>,
}

pub enum FormField {
    Title,
    Category,
    Priority,
    DueDate,
}
```

---

## 状态一致性保证

| 约束 | 实现方式 |
|------|---------|
| 内存与数据库一致 | 先写数据库成功，再更新内存缓存 |
| 表单验证 | Enter 提交前验证，失败时设置 error 字段，不关闭弹窗 |
| 选中索引有效性 | 每次 todos 列表变更后调用 `clamp_selected_index()` |
| 模式互斥 | AppMode 枚举保证同一时刻只有一个活跃模式 |
| Panic 恢复 | 注册 panic hook，确保终端状态恢复 |
