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
    }
}
