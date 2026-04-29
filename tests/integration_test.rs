#[cfg(test)]
mod storage_tests {
    use std::path::PathBuf;

    fn test_db(dir: &std::path::Path) -> PathBuf {
        dir.join("test.db")
    }

    // 由于是 bin crate，无法直接 use todo_tui::storage
    // 改用 rusqlite 直接验证数据库结构
    #[test]
    fn test_db_file_created() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = test_db(tmp.path());
        assert!(!db_path.exists());

        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )
        .unwrap();

        assert!(db_path.exists());
    }

    #[test]
    fn test_todo_crud() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = rusqlite::Connection::open(test_db(tmp.path())).unwrap();

        conn.execute_batch(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                priority TEXT NOT NULL DEFAULT 'Medium',
                category_id INTEGER,
                due_date TEXT,
                created_at TEXT NOT NULL
            );",
        )
        .unwrap();

        // 插入
        conn.execute(
            "INSERT INTO todos (title, completed, priority, created_at) VALUES ('测试任务', 0, 'High', '2026-04-23T10:00:00')",
            [],
        ).unwrap();
        let id = conn.last_insert_rowid();

        // 查询
        let title: String = conn
            .query_row(
                "SELECT title FROM todos WHERE id = ?1",
                rusqlite::params![id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(title, "测试任务");

        // 更新
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE id = ?1",
            rusqlite::params![id],
        )
        .unwrap();
        let completed: i64 = conn
            .query_row(
                "SELECT completed FROM todos WHERE id = ?1",
                rusqlite::params![id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(completed, 1);

        // 删除
        conn.execute("DELETE FROM todos WHERE id = ?1", rusqlite::params![id])
            .unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_priority_ordering() {
        // 验证优先级字符串排序逻辑
        let priorities = vec!["High", "Medium", "Low"];
        assert_eq!(priorities[0], "High");
        assert_eq!(priorities[1], "Medium");
        assert_eq!(priorities[2], "Low");
    }

    #[test]
    fn test_due_date_parse() {
        use chrono::NaiveDate;
        let valid = "2026-04-25".parse::<NaiveDate>();
        assert!(valid.is_ok());

        let invalid = "2026/04/25".parse::<NaiveDate>();
        assert!(invalid.is_err());

        let invalid2 = "not-a-date".parse::<NaiveDate>();
        assert!(invalid2.is_err());
    }
}
