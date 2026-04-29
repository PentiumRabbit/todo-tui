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
        let msg = Paragraph::new(app.t().terminal_too_small())
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
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(size);

    // 左侧标签栏 16 列，右侧列表占余下空间
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(16), Constraint::Min(0)])
        .split(chunks[0]);

    tags::render_tag_panel(frame, app, body[0]);
    list::render_list(frame, app, body[1]);

    render_status_bar(frame, app, chunks[1]);

    match app.mode {
        AppMode::Detail => detail::render_detail_popup(frame, app, size),
        AppMode::Add | AppMode::Edit => form::render_form(frame, app, size),
        AppMode::DeleteConfirm => render_delete_confirm(frame, app, size),
        AppMode::Help => help::render_help(frame, app, size),
        _ => {}
    }

    (body[0], body[1])
}

fn render_status_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    let t = app.t();
    let search_hint;
    let sort_hint;
    let hints = if app.focus_tag_panel {
        t.statusbar_tag_panel()
    } else {
        match app.mode {
            AppMode::Normal => {
                sort_hint = t.statusbar_normal(app.sort_order.label(&t));
                &sort_hint
            }
            AppMode::Search => {
                search_hint = t.statusbar_search(&app.search_query);
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
    let t = app.t();
    let title = app.selected_todo().map(|t| t.title.as_str()).unwrap_or("");
    let msg = t.delete_confirm_msg(title);
    let (confirm_label, cancel_label) = t.delete_confirm_hint();

    let popup = centered_rect(60, 7, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(t.delete_confirm_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(msg, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                confirm_label,
                Style::default()
                    .fg(theme::ACTION_CONFIRM)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                cancel_label,
                Style::default()
                    .fg(theme::ACTION_CANCEL)
                    .add_modifier(Modifier::BOLD),
            ),
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
