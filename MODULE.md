# 模块规范 (MODULE.md)

适用范围：`src/` 下所有模块

---

## 模块职责边界

| 模块 | 允许做 | 禁止做 |
|------|--------|--------|
| `models/` | 定义数据结构、枚举、纯计算方法 | I/O、数据库、UI 渲染 |
| `storage/` | SQLite CRUD、迁移 | 直接访问 AppState、UI 渲染 |
| `app.rs` | 持有状态、处理事件、调用 storage | 直接渲染 ratatui widget、持有数据库连接 |
| `ui/` | 将 AppState 渲染为 ratatui 组件 | 修改 AppState 任何字段、直接调用 storage |

---

## 命名约定

- 类型名：`UpperCamelCase`
- 函数/变量：`snake_case`
- 常量：`SCREAMING_SNAKE_CASE`
- 模块文件：`snake_case.rs`

---

## 测试约定

- 单元测试放在同文件底部 `#[cfg(test)] mod tests { ... }`
- 集成测试放在 `tests/` 目录
- 测试函数命名：`test_<被测函数>_<场景>`，例如 `test_filtered_todos_by_tag`
- Storage 集成测试必须使用 `tempfile::NamedTempFile` 隔离数据库

---

## 错误处理约定

- 所有公有函数返回 `anyhow::Result<T>`（除纯计算函数外）
- 禁止 `.unwrap()` / `.expect()`（测试代码除外）
- Storage 操作失败由 `AppState` 捕获后写入 `error_message`，由 UI 层展示

---

## 颜色/样式约定

- 所有颜色常量定义在 `src/ui/theme.rs`
- UI 文件通过 `use crate::ui::theme::*` 引用
- 禁止在 `list.rs` / `detail.rs` / `form.rs` / `tags.rs` 中出现 `Color::Rgb(...)` 字面量
