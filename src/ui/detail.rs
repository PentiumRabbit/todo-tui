use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{tag_color, AppState};
use crate::models::{Priority, Todo, TodoStatus};
use crate::ui::{centered_rect, theme};

/// 渲染 Todo 详情弹窗。
pub fn render_detail_popup(frame: &mut Frame, app: &AppState, area: Rect) {
    let Some(todo) = app.selected_todo() else {
        return;
    };

    let t = app.t();
    let popup = centered_rect(70, 20, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(t.detail_title())
        .borders(Borders::ALL)
        .border_type(theme::BORDER_TYPE)
        .border_style(theme::border_active());

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = build_detail_lines(todo, &t);
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn build_detail_lines<'a>(todo: &'a Todo, t: &'a crate::i18n::T) -> Vec<Line<'a>> {
    let label = Style::default().fg(theme::FG_LABEL);
    let value = Style::default().fg(theme::FG_VALUE);
    let bold = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let divider_style = Style::default().fg(theme::FG_DIVIDER);
    let divider_text = "  ─────────────────────────────────────";

    let (status_text, status_style) = status_display(todo, t);
    let (priority_icon, priority_style) = priority_display(&todo.priority, t);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(t.detail_label_title(), label),
            Span::styled(todo.title.clone(), bold),
        ]),
        Line::from(""),
        Line::from(Span::styled(divider_text, divider_style)),
        Line::from(""),
        Line::from(vec![
            Span::styled(t.detail_label_status(), label),
            Span::styled(status_text, status_style),
        ]),
        Line::from(vec![
            Span::styled(t.detail_label_priority(), label),
            Span::styled(priority_icon, priority_style),
        ]),
        tag_line(todo, label, t),
        due_line(todo, label, value, t),
    ];

    if let Some(notes) = todo.notes.as_deref() {
        if !notes.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(t.detail_label_notes(), label),
                Span::styled(notes.to_string(), value),
            ]));
        }
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(divider_text, divider_style)),
        Line::from(""),
        Line::from(vec![
            Span::styled(t.detail_label_created(), label),
            Span::styled(
                todo.created_at.replace('T', " "),
                Style::default().fg(theme::FG_MUTED),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t.detail_hint(),
            Style::default().fg(theme::FG_HINT),
        )]),
    ]);

    lines
}

fn status_display<'a>(todo: &Todo, t: &'a crate::i18n::T) -> (&'a str, Style) {
    match todo.status {
        TodoStatus::Pending if todo.is_overdue() => (
            t.status_pending_overdue(),
            Style::default().fg(theme::STATUS_OVERDUE),
        ),
        TodoStatus::Pending => (
            t.status_pending(),
            Style::default().fg(theme::STATUS_PENDING),
        ),
        TodoStatus::Done => (t.status_done(), Style::default().fg(theme::STATUS_DONE)),
        TodoStatus::Cancelled => (
            t.status_cancelled(),
            Style::default().fg(theme::STATUS_CANCELLED),
        ),
    }
}

fn priority_display<'a>(priority: &Priority, t: &'a crate::i18n::T) -> (&'a str, Style) {
    match priority {
        Priority::High => (
            t.priority_high(),
            Style::default()
                .fg(theme::PRIORITY_HIGH)
                .add_modifier(Modifier::BOLD),
        ),
        Priority::Medium => (
            t.priority_medium(),
            Style::default().fg(theme::PRIORITY_MEDIUM),
        ),
        Priority::Low => (t.priority_low(), Style::default().fg(theme::PRIORITY_LOW)),
    }
}

fn tag_line(todo: &Todo, label: Style, t: &crate::i18n::T) -> Line<'static> {
    if todo.tags.is_empty() {
        Line::from(vec![
            Span::styled(t.detail_label_tags(), label),
            Span::styled(t.detail_no_tags(), Style::default().fg(theme::FG_MUTED)),
        ])
    } else {
        let mut spans = vec![Span::styled(t.detail_label_tags(), label)];
        for tag in &todo.tags {
            let (r, g, b) = tag_color(tag);
            spans.push(Span::styled(
                format!("[{}] ", tag),
                Style::default()
                    .fg(ratatui::style::Color::Rgb(r, g, b))
                    .add_modifier(Modifier::BOLD),
            ));
        }
        Line::from(spans)
    }
}

fn due_line(todo: &Todo, label: Style, value: Style, t: &crate::i18n::T) -> Line<'static> {
    let Some(d) = todo.due_date.clone() else {
        return Line::from(vec![
            Span::styled(t.detail_label_due(), label),
            Span::styled(t.detail_no_due(), Style::default().fg(theme::FG_MUTED)),
        ]);
    };

    if todo.is_overdue() {
        Line::from(vec![
            Span::styled(t.detail_label_due(), label),
            Span::styled(d, theme::style_overdue_bold()),
            Span::styled(
                t.detail_overdue_suffix(),
                Style::default().fg(theme::STATUS_OVERDUE),
            ),
        ])
    } else if todo.is_due_today() {
        Line::from(vec![
            Span::styled(t.detail_label_due(), label),
            Span::styled(d, theme::style_due_today_bold()),
            Span::styled(
                t.detail_due_today_suffix(),
                Style::default().fg(theme::STATUS_DUE_TODAY),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(t.detail_label_due(), label),
            Span::styled(d, value),
        ])
    }
}
