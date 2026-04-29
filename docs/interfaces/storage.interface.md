# storage 模块接口契约

**架构师**: 架构师角色
**日期**: 2026-04-23
**Step**: 3/6

---

## Storage 结构

```rust
pub struct Storage {
    conn: Connection,  // rusqlite::Connection
}
```

---

## 对外方法

### `Storage::new(db_path: &Path) -> Result<Storage>`
- 创建或打开数据库，执行 schema 迁移
- 异常：文件权限不足、磁盘满时返回 Err

### Todo 操作

```rust
pub fn insert_todo(&self, todo: &NewTodo) -> Result<Todo>
pub fn update_todo(&self, todo: &Todo) -> Result<()>
pub fn delete_todo(&self, id: i64) -> Result<()>
pub fn list_todos(&self) -> Result<Vec<Todo>>
pub fn list_todos_by_category(&self, category_id: Option<i64>) -> Result<Vec<Todo>>
```

### Category 操作

```rust
pub fn insert_category(&self, name: &str) -> Result<Category>
pub fn delete_category(&self, id: i64) -> Result<()>
pub fn list_categories(&self) -> Result<Vec<Category>>
```

---

## 调用约束

- 所有方法同步执行（rusqlite 不支持异步）
- 写操作立即提交（不使用显式事务，单条写入）
- 删除分类时自动将关联 todo 的 category_id 置为 NULL（由 SQLite FOREIGN KEY ON DELETE SET NULL 保证）

---

## Schema

```sql
CREATE TABLE IF NOT EXISTS todos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    priority    TEXT NOT NULL DEFAULT 'Medium',
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    due_date    TEXT,          -- ISO 8601: YYYY-MM-DD
    created_at  TEXT NOT NULL  -- ISO 8601: YYYY-MM-DDTHH:MM:SS
);

CREATE TABLE IF NOT EXISTS categories (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER NOT NULL
);
```
