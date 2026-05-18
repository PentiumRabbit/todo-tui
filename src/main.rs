mod app;
mod config;
mod i18n;
mod models;
mod storage;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use app::AppState;
use config::Config;
use models::{FormState, NewTodo, Priority};
use storage::Storage;

fn db_path() -> PathBuf {
    dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("todo-tui")
        .join("todos.db")
}

struct CliAddArgs {
    title: String,
    priority: Priority,
    tags: Vec<String>,
    due_date: Option<String>,
}

/// 解析 `add` 子命令参数，返回 Ok 或描述错误的字符串。
fn parse_cli_add_args(args: &[String]) -> std::result::Result<CliAddArgs, String> {
    let mut title: Option<String> = None;
    let mut priority = Priority::Medium;
    let mut tags: Vec<String> = Vec::new();
    let mut due_date: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" => {
                i += 1;
                let val = args.get(i).map(|s| s.as_str()).unwrap_or("");
                priority = match val.to_lowercase().as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    "low" => Priority::Low,
                    other => {
                        return Err(format!(
                            "错误：-p 值无效 '{other}'，必须为 high / medium / low"
                        ));
                    }
                };
            }
            "-t" => {
                i += 1;
                match args.get(i) {
                    Some(val) if !val.starts_with('-') => tags.push(val.clone()),
                    _ => return Err("错误：-t 后需要标签值".to_string()),
                }
            }
            "-d" => {
                i += 1;
                let val = args.get(i).map(|s| s.as_str()).unwrap_or("");
                match FormState::parse_due_date(val) {
                    Some(dt) => {
                        due_date = Some(dt.format("%Y-%m-%d %H:%M").to_string());
                    }
                    None => {
                        return Err(format!(
                            "错误：-d 日期格式无效 '{val}'，需要 YYYY-MM-DD HH:MM"
                        ));
                    }
                }
            }
            flag if flag.starts_with('-') => {
                return Err(format!("错误：未知参数 '{flag}'"));
            }
            val => {
                if title.is_none() {
                    title = Some(val.to_string());
                }
            }
        }
        i += 1;
    }

    let title = match title {
        Some(t) if !t.is_empty() => t,
        _ => {
            return Err(
                "用法：todo-tui add <title> [-p high|medium|low] [-t tag]... [-d 'YYYY-MM-DD HH:MM']"
                    .to_string(),
            );
        }
    };

    Ok(CliAddArgs {
        title,
        priority,
        tags,
        due_date,
    })
}

fn run_cli_add(args: &[String]) -> Result<()> {
    let parsed = parse_cli_add_args(args).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let storage = Storage::new(&db_path())?;
    let todo = storage.insert_todo(&NewTodo {
        title: parsed.title,
        priority: parsed.priority,
        tags: parsed.tags,
        due_date: parsed.due_date,
        notes: None,
    })?;
    println!("已添加 #{}: {}", todo.id, todo.title);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_title_only() {
        let r = parse_cli_add_args(&s(&["买咖啡"])).unwrap();
        assert_eq!(r.title, "买咖啡");
        assert_eq!(r.priority, Priority::Medium);
        assert!(r.tags.is_empty());
        assert!(r.due_date.is_none());
    }

    #[test]
    fn test_parse_no_title_returns_err() {
        assert!(parse_cli_add_args(&s(&[])).is_err());
    }

    #[test]
    fn test_parse_empty_title_returns_err() {
        assert!(parse_cli_add_args(&s(&[""])).is_err());
    }

    #[test]
    fn test_parse_priority_high() {
        let r = parse_cli_add_args(&s(&["x", "-p", "high"])).unwrap();
        assert_eq!(r.priority, Priority::High);
    }

    #[test]
    fn test_parse_priority_case_insensitive() {
        let r = parse_cli_add_args(&s(&["x", "-p", "HIGH"])).unwrap();
        assert_eq!(r.priority, Priority::High);
    }

    #[test]
    fn test_parse_priority_invalid_returns_err() {
        assert!(parse_cli_add_args(&s(&["x", "-p", "critical"])).is_err());
    }

    #[test]
    fn test_parse_tags_multiple() {
        let r = parse_cli_add_args(&s(&["x", "-t", "work", "-t", "urgent"])).unwrap();
        assert_eq!(r.tags, vec!["work", "urgent"]);
    }

    #[test]
    fn test_parse_due_date_valid() {
        let r = parse_cli_add_args(&s(&["x", "-d", "2026-05-31 18:00"])).unwrap();
        assert_eq!(r.due_date, Some("2026-05-31 18:00".to_string()));
    }

    #[test]
    fn test_parse_due_date_invalid_returns_err() {
        assert!(parse_cli_add_args(&s(&["x", "-d", "31/05/2026"])).is_err());
    }

    #[test]
    fn test_parse_combined_flags() {
        let r = parse_cli_add_args(&s(&[
            "发布 v1.2",
            "-p",
            "high",
            "-t",
            "release",
            "-d",
            "2026-05-20 10:00",
        ]))
        .unwrap();
        assert_eq!(r.title, "发布 v1.2");
        assert_eq!(r.priority, Priority::High);
        assert_eq!(r.tags, vec!["release"]);
        assert_eq!(r.due_date, Some("2026-05-20 10:00".to_string()));
    }

    #[test]
    fn test_parse_unknown_flag_returns_err() {
        assert!(parse_cli_add_args(&s(&["x", "--verbose"])).is_err());
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "add" {
        return run_cli_add(&args[2..]);
    }

    // 注册 panic hook，确保终端恢复
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    let storage = Storage::new(&db_path())?;
    let cfg = Config::load()?;
    let mut app = AppState::new(storage, cfg)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut AppState) -> Result<()> {
    use ratatui::layout::Rect;
    use std::cell::Cell;
    use std::time::SystemTime;
    use ui::FormAreas;

    // 保存上一帧布局区域，供鼠标命中检测使用（初始为零区域，鼠标事件在首帧后才到来）
    let last_tag_area = Cell::new(Rect::default());
    let last_list_area = Cell::new(Rect::default());
    let last_form_areas: Cell<FormAreas> = Cell::new(FormAreas::default());

    // mtime 轮询：每约 500ms（31 × 16ms）检测一次 DB 文件修改时间
    let mut mtime_tick: u32 = 0;
    let mut last_mtime: Option<SystemTime> = None;
    let db = db_path();

    loop {
        terminal.draw(|f| {
            let (tag_area, list_area, form_areas) = ui::render(f, app);
            last_tag_area.set(tag_area);
            last_list_area.set(list_area);
            last_form_areas.set(form_areas);
        })?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => match app.handle_event(key)? {
                    models::AppAction::Quit => break,
                    models::AppAction::Continue => {}
                },
                Event::Mouse(mouse) => {
                    app.handle_mouse(
                        mouse,
                        last_tag_area.get(),
                        last_list_area.get(),
                        last_form_areas.get(),
                    )?;
                }
                _ => {}
            }
        }

        // 每约 500ms 检测 DB 文件 mtime 变化
        mtime_tick += 1;
        if mtime_tick >= 31 {
            mtime_tick = 0;
            if let Ok(meta) = std::fs::metadata(&db) {
                if let Ok(mtime) = meta.modified() {
                    let changed = match last_mtime {
                        None => true,
                        Some(prev) => mtime != prev,
                    };
                    if changed {
                        last_mtime = Some(mtime);
                        app.trigger_reload();
                    }
                }
            }
        }
    }
    Ok(())
}
