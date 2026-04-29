mod detail;
mod form;
mod help;
mod list;
mod tags;
pub mod theme;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::models::AppMode;

/// 渲染整帧，返回 (tag_panel_area, list_area) 供调用方用于鼠标命中检测。
pub fn render(frame: &mut Frame, app: &AppState) -> (Rect, Rect) {
    let size = frame.area();

    if size.width < 80 || size.height < 24 {
        let msg = Paragraph::new("终端太小，请调整至 80×24 以上")
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        frame.render_widget(msg, size);
        return (Rect::default(), Rect::default());
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    render_title_bar(frame, app, chunks[0]);

    // 左侧标签栏 16 列，右侧列表占余下空间
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(16), Constraint::Min(0)])
        .split(chunks[1]);

    tags::render_tag_panel(frame, app, body[0]);
    list::render_list(frame, app, body[1]);

    render_status_bar(frame, app, chunks[2]);

    match app.mode {
        AppMode::Detail => detail::render_detail_popup(frame, app, size),
        AppMode::Add | AppMode::Edit => form::render_form(frame, app, size),
        AppMode::DeleteConfirm => render_delete_confirm(frame, app, size),
        AppMode::Help => help::render_help(frame, size),
        _ => {}
    }

    (body[0], body[1])
}

fn render_title_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    let total = app.filtered_todos().len();
    let tag_label = app.selected_tag.as_deref().unwrap_or("全部");
    let title = format!(" todo-tui  [{}]  {} 条", tag_label, total);
    let hints = " [?] 帮助  [q] 退出 ";
    let width = area.width as usize;
    let pad = width.saturating_sub(title.len() + hints.len());
    let line = Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" ".repeat(pad)),
        Span::styled(hints, Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme::BG_TITLEBAR)),
        area,
    );
}

fn render_status_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    let search_hint;
    let hints = if app.focus_tag_panel {
        " [→/Tab] 切回列表  [j/k] 移动  [Enter] 选择标签 "
    } else {
        match app.mode {
            AppMode::Normal => " [←/Tab] 标签栏  [Enter] 详情  [a] 添加  [e] 编辑  [d] 删除  [Space] 完成  [x] 取消  [/] 搜索 ",
            AppMode::Search => {
                search_hint = format!(" 搜索: {}█  [Esc] 退出", app.search_query);
                &search_hint
            }
            _ => "",
        }
    };
    frame.render_widget(
        Paragraph::new(hints).style(
            Style::default()
                .bg(theme::BG_STATUSBAR)
                .fg(theme::FG_STATUSBAR),
        ),
        area,
    );
}

fn render_delete_confirm(frame: &mut Frame, app: &AppState, area: Rect) {
    let title = app.selected_todo().map(|t| t.title.as_str()).unwrap_or("");
    let msg = format!("确定要删除 \"{}\" 吗？", title);

    let popup = centered_rect(60, 7, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" 确认删除 ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(&msg, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  [y/Enter] ",
                Style::default()
                    .fg(theme::ACTION_CONFIRM)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("确认  "),
            Span::styled(
                "[n/Esc] ",
                Style::default()
                    .fg(theme::ACTION_CANCEL)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("取消"),
        ]),
    ];
    frame.render_widget(Paragraph::new(lines), inner);
}

/// 计算居中弹窗的 Rect，宽度为父区域百分比，高度固定。
pub fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect::new(x, y, popup_width.min(r.width), height.min(r.height))
}
