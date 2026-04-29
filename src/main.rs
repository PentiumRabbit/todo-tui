mod app;
mod models;
mod storage;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use app::AppState;
use storage::Storage;

fn db_path() -> PathBuf {
    dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".todo-tui")
        .join("todos.db")
}

fn main() -> Result<()> {
    // 注册 panic hook，确保终端恢复
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(info);
    }));

    let storage = Storage::new(&db_path())?;
    let mut app = AppState::new(storage)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;


        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    match app.handle_event(key)? {
                        models::AppAction::Quit => break,
                        models::AppAction::Continue => {}
                    }
                }
                Event::Mouse(mouse) => {
                    app.handle_mouse(mouse)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
