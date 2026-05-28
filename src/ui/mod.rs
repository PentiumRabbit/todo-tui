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
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::models::AppMode;
pub use form::FormAreas;

/// 渲染整帧，返回 (tag_panel_area, list_area, form_areas) 供鼠标命中检测。
pub fn render(frame: &mut Frame, app: &AppState) -> (Rect, Rect, FormAreas) {
    let size = frame.area();

    if size.width < 80 || size.height < 24 {
        let msg = Paragraph::new(app.t().terminal_too_small())
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(theme::BORDER_TYPE),
            );
        frame.render_widget(msg, size);
        return (Rect::default(), Rect::default(), FormAreas::default());
    }

    let show_bottom_bar = app.config.show_statusbar || matches!(app.mode, AppMode::Search);
    let statusbar_height = if show_bottom_bar { 1 } else { 0 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(statusbar_height)])
        .split(size);

    let (tag_area, list_area) = if app.show_filter_panel {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(16), Constraint::Min(0)])
            .split(chunks[0]);
        tags::render_tag_panel(frame, app, body[0]);
        list::render_list(frame, app, body[1]);
        (body[0], body[1])
    } else {
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0)])
            .split(chunks[0]);
        list::render_list(frame, app, body[0]);
        (Rect::default(), body[0])
    };

    if show_bottom_bar {
        render_status_bar(frame, app, chunks[1]);
    }

    let form_areas = match app.mode {
        AppMode::Detail => {
            detail::render_detail_popup(frame, app, size);
            FormAreas::default()
        }
        AppMode::Add | AppMode::Edit => form::render_form(frame, app, size),
        AppMode::DeleteConfirm => {
            render_delete_confirm(frame, app, size);
            FormAreas::default()
        }
        AppMode::Help => {
            help::render_help(frame, app, size);
            FormAreas::default()
        }
        _ => FormAreas::default(),
    };

    (tag_area, list_area, form_areas)
}

fn render_status_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    // regex mode: render colored multi-span line and return early
    if matches!(app.mode, AppMode::Search) && app.regex_mode {
        let re_error_prefix = "[RE:ERROR] ";
        let re_prefix = "[RE] ";
        let spans = if let Some(err) = &app.regex_error {
            let max_content = area.width.saturating_sub(re_error_prefix.len() as u16) as usize;
            let truncated = truncate_error_msg(err, max_content);
            vec![
                Span::styled(re_error_prefix, Style::default().fg(theme::ACTION_ERROR)),
                Span::styled(truncated, Style::default().fg(theme::ACTION_ERROR)),
            ]
        } else {
            vec![
                Span::styled(re_prefix, Style::default().fg(theme::FG_REGEX_INDICATOR)),
                Span::raw(app.search_query.clone()),
            ]
        };
        frame.render_widget(
            Paragraph::new(Line::from(spans)).style(
                Style::default()
                    .bg(theme::BG_STATUSBAR)
                    .fg(theme::FG_STATUSBAR),
            ),
            area,
        );
        return;
    }

    let t = app.t();
    let search_hint;
    let normal_hint;
    let hints = if app.focus_tag_panel {
        t.statusbar_tag_panel()
    } else {
        match app.mode {
            AppMode::Normal => {
                normal_hint = t.statusbar_normal("");
                &normal_hint
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

fn truncate_error_msg(s: &str, max_width: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    if max_width == 0 {
        return String::new();
    }
    let mut width = 0usize;
    let mut end = s.len();
    let mut truncated = false;
    for (i, c) in s.char_indices() {
        let cw = c.width().unwrap_or(0);
        // reserve 1 column for ellipsis if there is more content
        if width + cw > max_width.saturating_sub(1) {
            if i < s.len() {
                end = i;
                truncated = true;
            }
            break;
        }
        width += cw;
        end = i + c.len_utf8();
    }
    if truncated {
        format!("{}…", &s[..end])
    } else {
        s.to_string()
    }
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
        .border_type(theme::BORDER_TYPE)
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
