use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;
use std::str::FromStr;

use crate::models::{NewTodo, Priority, Todo, TodoStatus};

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let storage = Self { conn };
        storage.migrate()?;
        Ok(storage)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_version (version INTEGER NOT NULL);
             CREATE TABLE IF NOT EXISTS categories (
                 id   INTEGER PRIMARY KEY AUTOINCREMENT,
                 name TEXT NOT NULL UNIQUE
             );
             CREATE TABLE IF NOT EXISTS todos (
                 id          INTEGER PRIMARY KEY AUTOINCREMENT,
                 title       TEXT NOT NULL,
                 completed   INTEGER NOT NULL DEFAULT 0,
                 priority    TEXT NOT NULL DEFAULT 'Medium',
                 category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
                 due_date    TEXT,
                 created_at  TEXT NOT NULL
             );",
        )?;

        let version: i64 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);

        if version < 1 {
            self.conn.execute_batch(
                "INSERT OR IGNORE INTO categories (name) VALUES ('工作'), ('生活'), ('学习');
                 INSERT OR REPLACE INTO schema_version (version) VALUES (1);",
            )?;
        }

        if version < 2 {
            self.conn.execute_batch(
                "ALTER TABLE todos ADD COLUMN status TEXT NOT NULL DEFAULT 'Pending';
                 UPDATE todos SET status = CASE WHEN completed = 1 THEN 'Done' ELSE 'Pending' END;
                 INSERT OR REPLACE INTO schema_version (version) VALUES (2);",
            )?;
        }

        // v3: 多标签支持，新增 tags 表和 todo_tags 关联表
        if version < 3 {
            self.conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS tags (
                     id   INTEGER PRIMARY KEY AUTOINCREMENT,
                     name TEXT NOT NULL UNIQUE
                 );
                 CREATE TABLE IF NOT EXISTS todo_tags (
                     todo_id INTEGER NOT NULL REFERENCES todos(id) ON DELETE CASCADE,
                     tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                     PRIMARY KEY (todo_id, tag_id)
                 );
                 INSERT OR IGNORE INTO tags (name)
                     SELECT DISTINCT name FROM categories WHERE name IN ('工作','生活','学习');
                 INSERT OR IGNORE INTO todo_tags (todo_id, tag_id)
                     SELECT t.id, tg.id FROM todos t
                     JOIN categories c ON c.id = t.category_id
                     JOIN tags tg ON tg.name = c.name
                     WHERE t.category_id IS NOT NULL;
                 INSERT OR REPLACE INTO schema_version (version) VALUES (3);",
            )?;
        }

        Ok(())
    }

    pub fn insert_todo(&self, new_todo: &NewTodo) -> Result<Todo> {
        let created_at = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO todos (title, status, priority, due_date, created_at)
             VALUES (?1, 'Pending', ?2, ?3, ?4)",
            params![
                new_todo.title,
                new_todo.priority.as_str(),
                new_todo.due_date,
                created_at
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        let tags = self.sync_tags(id, &new_todo.tags)?;
        Ok(Todo {
            id,
            title: new_todo.title.clone(),
            status: TodoStatus::Pending,
            priority: new_todo.priority.clone(),
            tags,
            due_date: new_todo.due_date.clone(),
            created_at,
        })
    }

    pub fn update_todo(&self, todo: &Todo) -> Result<()> {
        self.conn.execute(
            "UPDATE todos SET title=?1, status=?2, priority=?3, due_date=?4 WHERE id=?5",
            params![
                todo.title,
                todo.status.as_str(),
                todo.priority.as_str(),
                todo.due_date,
                todo.id
            ],
        )?;
        self.sync_tags(todo.id, &todo.tags)?;
        Ok(())
    }

    /// 同步标签：在事务内清空旧关联并写入新标签，保证原子性。
    fn sync_tags(&self, todo_id: i64, tags: &[String]) -> Result<Vec<String>> {
        self.conn.execute_batch("BEGIN;")?;
        let result = (|| -> Result<()> {
            self.conn
                .execute("DELETE FROM todo_tags WHERE todo_id = ?1", params![todo_id])?;
            for name in tags {
                let name = name.trim();
                if name.is_empty() {
                    continue;
                }
                self.conn.execute(
                    "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                    params![name],
                )?;
                let tag_id: i64 = self.conn.query_row(
                    "SELECT id FROM tags WHERE name = ?1",
                    params![name],
                    |r| r.get(0),
                )?;
                self.conn.execute(
                    "INSERT OR IGNORE INTO todo_tags (todo_id, tag_id) VALUES (?1, ?2)",
                    params![todo_id, tag_id],
                )?;
            }
            Ok(())
        })();
        match result {
            Ok(()) => {
                self.conn.execute_batch("COMMIT;")?;
            }
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK;");
                return Err(e);
            }
        }
        Ok(tags
            .iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }

    pub fn delete_todo(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM todos WHERE id=?1", params![id])?;
        Ok(())
    }

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, status, priority, due_date, created_at
             FROM todos ORDER BY
                 CASE status WHEN 'Pending' THEN 0 ELSE 1 END ASC,
                 CASE priority WHEN 'High' THEN 0 WHEN 'Medium' THEN 1 ELSE 2 END ASC,
                 created_at DESC",
        )?;
        let mut todos = stmt
            .query_map([], |row| {
                let status_str: String = row.get(2)?;
                let priority_str: String = row.get(3)?;
                let status = TodoStatus::from_str(&status_str).map_err(|_| {
                    rusqlite::Error::InvalidColumnName(format!("unknown status: {status_str}"))
                })?;
                let priority = Priority::from_str(&priority_str).map_err(|_| {
                    rusqlite::Error::InvalidColumnName(format!("unknown priority: {priority_str}"))
                })?;
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    status,
                    priority,
                    tags: Vec::new(),
                    due_date: row.get(4)?,
                    created_at: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        // 批量加载所有 todo 的标签
        let mut tag_stmt = self.conn.prepare(
            "SELECT tt.todo_id, tg.name FROM todo_tags tt
             JOIN tags tg ON tg.id = tt.tag_id
             ORDER BY tt.todo_id, tg.name",
        )?;
        let pairs = tag_stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        for (todo_id, tag_name) in pairs {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == todo_id) {
                todo.tags.push(tag_name);
            }
        }

        Ok(todos)
    }

    pub fn list_all_tags(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM tags ORDER BY name")?;
        let tags = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        Ok(tags)
    }
}
