use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
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

    let popup = centered_rect(70, 20, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" 详情 ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_active());

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let lines = build_detail_lines(todo);
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn build_detail_lines(todo: &Todo) -> Vec<Line<'static>> {
    let label = Style::default().fg(theme::FG_LABEL);
    let value = Style::default().fg(theme::FG_VALUE);
    let bold = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let divider_style = Style::default().fg(theme::FG_DIVIDER);
    let divider_text = "  ─────────────────────────────────────";

    let (status_text, status_style) = status_display(todo);
    let (priority_icon, priority_style) = priority_display(&todo.priority);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  标题:      ", label),
            Span::styled(todo.title.clone(), bold),
        ]),
        Line::from(""),
        Line::from(Span::styled(divider_text, divider_style)),
        Line::from(""),
        Line::from(vec![
            Span::styled("  状态:      ", label),
            Span::styled(status_text, status_style),
        ]),
        Line::from(vec![
            Span::styled("  优先级:    ", label),
            Span::styled(priority_icon, priority_style),
        ]),
        tag_line(todo, label),
        due_line(todo, label, value),
    ];

    if let Some(notes) = todo.notes.as_deref() {
        if !notes.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  备注:      ", label),
                Span::styled(notes.to_string(), value),
            ]));
        }
    }

    lines.extend([
        Line::from(""),
        Line::from(Span::styled(divider_text, divider_style)),
        Line::from(""),
        Line::from(vec![
            Span::styled("  创建时间:  ", label),
            Span::styled(
                todo.created_at.replace('T', " "),
                Style::default().fg(theme::FG_MUTED),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  [Esc] 关闭  [e] 编辑  [d] 删除  [Space] 切换完成",
            Style::default().fg(theme::FG_HINT),
        )]),
    ]);

    lines
}

fn status_display(todo: &Todo) -> (&'static str, Style) {
    match todo.status {
        TodoStatus::Pending if todo.is_overdue() => (
            "□ 未完成（已过期）",
            Style::default().fg(theme::STATUS_OVERDUE),
        ),
        TodoStatus::Pending => ("□ 未完成", Style::default().fg(theme::STATUS_PENDING)),
        TodoStatus::Done => ("✓ 已完成", Style::default().fg(theme::STATUS_DONE)),
        TodoStatus::Cancelled => ("✗ 已取消", Style::default().fg(theme::STATUS_CANCELLED)),
    }
}

fn priority_display(priority: &Priority) -> (&'static str, Style) {
    match priority {
        Priority::High => (
            "▲ 高",
            Style::default()
                .fg(theme::PRIORITY_HIGH)
                .add_modifier(Modifier::BOLD),
        ),
        Priority::Medium => ("● 中", Style::default().fg(theme::PRIORITY_MEDIUM)),
        Priority::Low => ("▼ 低", Style::default().fg(theme::PRIORITY_LOW)),
    }
}

fn tag_line(todo: &Todo, label: Style) -> Line<'static> {
    if todo.tags.is_empty() {
        Line::from(vec![
            Span::styled("  标签:      ", label),
            Span::styled("无", Style::default().fg(theme::FG_MUTED)),
        ])
    } else {
        let mut spans = vec![Span::styled("  标签:      ", label)];
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

fn due_line(todo: &Todo, label: Style, value: Style) -> Line<'static> {
    let Some(d) = todo.due_date.clone() else {
        return Line::from(vec![
            Span::styled("  截止日期:  ", label),
            Span::styled("未设置", Style::default().fg(theme::FG_MUTED)),
        ]);
    };

    if todo.is_overdue() {
        Line::from(vec![
            Span::styled("  截止日期:  ", label),
            Span::styled(d, theme::style_overdue_bold()),
            Span::styled("  ⚠ 已过期", Style::default().fg(theme::STATUS_OVERDUE)),
        ])
    } else if todo.is_due_today() {
        Line::from(vec![
            Span::styled("  截止日期:  ", label),
            Span::styled(d, theme::style_due_today_bold()),
            Span::styled("  ⚠ 今天到期", Style::default().fg(theme::STATUS_DUE_TODAY)),
        ])
    } else {
        Line::from(vec![
            Span::styled("  截止日期:  ", label),
            Span::styled(d, value),
        ])
    }
}
