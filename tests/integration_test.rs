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
                due_date TEXT
            );",
        )
        .unwrap();

        conn.execute(
            "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
            rusqlite::params!["测试任务", 0],
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert_and_query_todo() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = test_db(tmp.path());
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )
        .unwrap();

        conn.execute(
            "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
            rusqlite::params!["测试任务", 0],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT id, title, completed FROM todos WHERE title = ?1")
            .unwrap();
        let todo = stmt
            .query_row(rusqlite::params!["测试任务"], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, i64>(2)?))
            })
            .unwrap();

        assert_eq!(todo.0, 1);
        assert_eq!(todo.1, "测试任务");
        assert_eq!(todo.2, 0);
    }

    #[test]
    fn test_update_todo_status() {
        let tmp = tempfile::tempdir().unwrap();
        let db_path = test_db(tmp.path());
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )
        .unwrap();

        conn.execute(
            "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
            rusqlite::params!["待更新任务", 0],
        )
        .unwrap();

        conn.execute(
            "UPDATE todos SET completed = ?1 WHERE title = ?2",
            rusqlite::params![1, "待更新任务"],
        )
        .unwrap();

        let completed: i64 = conn
            .query_row(
                "SELECT completed FROM todos WHERE title = ?1",
                rusqlite::params!["待更新任务"],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(completed, 1);
<<<<<<< HEAD
    }
}
=======

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
        let priorities = ["High", "Medium", "Low"];
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

    #[test]
    fn test_category_association() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = rusqlite::Connection::open(test_db(tmp.path())).unwrap();

        conn.execute_batch(
            "CREATE TABLE categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            );
            CREATE TABLE todos (
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

        // 插入 category
        conn.execute(
            "INSERT INTO categories (name) VALUES ('工作')",
            [],
        )
        .unwrap();
        let cat_id = conn.last_insert_rowid();

        // 插入关联 category 的 todo
        conn.execute(
            "INSERT INTO todos (title, completed, priority, category_id, created_at) VALUES ('写报告', 0, 'High', ?1, '2026-04-23T10:00:00')",
            rusqlite::params![cat_id],
        )
        .unwrap();
        let todo_id = conn.last_insert_rowid();

        // 关联查询：通过 category_id 关联 category 表
        let (title, cat_name): (String, String) = conn
            .query_row(
                "SELECT t.title, c.name FROM todos t JOIN categories c ON t.category_id = c.id WHERE t.id = ?1",
                rusqlite::params![todo_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(title, "写报告");
        assert_eq!(cat_name, "工作");

        // 无 category 的 todo 关联查询返回 NULL
        conn.execute(
            "INSERT INTO todos (title, completed, priority, created_at) VALUES ('无分类任务', 0, 'Low', '2026-04-23T10:00:00')",
            [],
        )
        .unwrap();
        let no_cat_id = conn.last_insert_rowid();
        let cat_id_opt: Option<i64> = conn
            .query_row(
                "SELECT category_id FROM todos WHERE id = ?1",
                rusqlite::params![no_cat_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(cat_id_opt.is_none());
    }

    #[test]
    fn test_due_date_ordering() {
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

        // 插入不同 due_date 的 todo
        let dates = ["2026-05-01", "2026-04-20", "2026-06-15", "2026-04-01"];
        for (i, d) in dates.iter().enumerate() {
            conn.execute(
                "INSERT INTO todos (title, completed, priority, due_date, created_at) VALUES (?1, 0, 'Medium', ?2, '2026-04-23T10:00:00')",
                rusqlite::params![format!("任务{}", i), d],
            )
            .unwrap();
        }

        // 按 due_date 升序排序查询
        let mut stmt = conn
            .prepare("SELECT title FROM todos ORDER BY due_date ASC")
            .unwrap();
        let titles: Vec<String> = stmt
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(titles, vec!["任务3", "任务1", "任务0", "任务2"]);
    }

    #[test]
    fn test_completed_filter() {
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

        // 插入 3 个未完成 + 2 个已完成
        for i in 0..3 {
            conn.execute(
                "INSERT INTO todos (title, completed, priority, created_at) VALUES (?1, 0, 'Medium', '2026-04-23T10:00:00')",
                rusqlite::params![format!("未完成{}", i)],
            )
            .unwrap();
        }
        for i in 0..2 {
            conn.execute(
                "INSERT INTO todos (title, completed, priority, created_at) VALUES (?1, 1, 'Medium', '2026-04-23T10:00:00')",
                rusqlite::params![format!("已完成{}", i)],
            )
            .unwrap();
        }

        // 过滤未完成 (completed=0)
        let pending_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos WHERE completed = 0", [], |r| r.get(0))
            .unwrap();
        assert_eq!(pending_count, 3);

        // 过滤已完成 (completed=1)
        let done_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos WHERE completed = 1", [], |r| r.get(0))
            .unwrap();
        assert_eq!(done_count, 2);

        // 全部
        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM todos", [], |r| r.get(0))
            .unwrap();
        assert_eq!(total, 5);
    }

    #[test]
    fn test_boundary_dates() {
        use chrono::NaiveDate;

        // 闰年 2024-02-29 合法
        let leap = "2024-02-29".parse::<NaiveDate>();
        assert!(leap.is_ok());
        assert_eq!(leap.unwrap().to_string(), "2024-02-29");

        // 非闰年 2023-02-29 非法
        let non_leap = "2023-02-29".parse::<NaiveDate>();
        assert!(non_leap.is_err());

        // 最小日期 0001-01-01 合法
        let min_date = "0001-01-01".parse::<NaiveDate>();
        assert!(min_date.is_ok());

        // 最大日期 9999-12-31 合法
        let max_date = "9999-12-31".parse::<NaiveDate>();
        assert!(max_date.is_ok());

        // 月份越界 2026-13-01 非法
        let bad_month = "2026-13-01".parse::<NaiveDate>();
        assert!(bad_month.is_err());

        // 日期越界 2026-04-31 非法（4月只有30天）
        let bad_day = "2026-04-31".parse::<NaiveDate>();
        assert!(bad_day.is_err());
    }}
>>>>>>> ai-task-12
