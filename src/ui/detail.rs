use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{AppState, tag_color};
use crate::models::{Priority, TodoStatus};
use crate::ui::centered_rect;

pub fn render_detail_popup(frame: &mut Frame, app: &AppState, area: Rect) {
    let Some(todo) = app.selected_todo() else { return };

    let popup = centered_rect(70, 20, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" 详情 ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let label = Style::default().fg(Color::Rgb(120, 120, 120));
    let value = Style::default().fg(Color::Rgb(220, 220, 220));
    let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);

    let (status_text, status_style) = match todo.status {
        TodoStatus::Pending if todo.is_overdue() => (
            "□ 未完成（已过期）",
            Style::default().fg(Color::Rgb(220, 100, 40)),
        ),
        TodoStatus::Pending => ("□ 未完成", Style::default().fg(Color::Rgb(200, 200, 200))),
        TodoStatus::Done => ("✓ 已完成", Style::default().fg(Color::Rgb(80, 200, 120))),
        TodoStatus::Cancelled => ("✗ 已取消", Style::default().fg(Color::Rgb(120, 120, 120))),
    };

    let (priority_icon, priority_style) = match todo.priority {
        Priority::High => ("▲ 高", Style::default().fg(Color::Rgb(240, 80, 80)).add_modifier(Modifier::BOLD)),
        Priority::Medium => ("● 中", Style::default().fg(Color::Rgb(220, 180, 60))),
        Priority::Low => ("▼ 低", Style::default().fg(Color::Rgb(80, 180, 80))),
    };

    let due_line: Line = if let Some(ref d) = todo.due_date {
        if todo.is_overdue() {
            Line::from(vec![
                Span::styled("  截止日期:  ", label),
                Span::styled(d.as_str(), Style::default().fg(Color::Rgb(220, 100, 40)).add_modifier(Modifier::BOLD)),
                Span::styled("  ⚠ 已过期", Style::default().fg(Color::Rgb(220, 100, 40))),
            ])
        } else if todo.is_due_today() {
            Line::from(vec![
                Span::styled("  截止日期:  ", label),
                Span::styled(d.as_str(), Style::default().fg(Color::Rgb(220, 180, 60)).add_modifier(Modifier::BOLD)),
                Span::styled("  ⚠ 今天到期", Style::default().fg(Color::Rgb(220, 180, 60))),
            ])
        } else {
            Line::from(vec![
                Span::styled("  截止日期:  ", label),
                Span::styled(d.as_str(), value),
            ])
        }
    } else {
        Line::from(vec![
            Span::styled("  截止日期:  ", label),
            Span::styled("未设置", Style::default().fg(Color::Rgb(80, 80, 80))),
        ])
    };

    // 标签行
    let tag_line = if todo.tags.is_empty() {
        Line::from(vec![
            Span::styled("  标签:      ", label),
            Span::styled("无", Style::default().fg(Color::Rgb(80, 80, 80))),
        ])
    } else {
        let mut spans = vec![Span::styled("  标签:      ", label)];
        for tag in &todo.tags {
            let (r, g, b) = tag_color(tag);
            spans.push(Span::styled(
                format!("[{}] ", tag),
                Style::default().fg(Color::Rgb(r, g, b)).add_modifier(Modifier::BOLD),
            ));
        }
        Line::from(spans)
    };

    let divider = Line::from(Span::styled(
        "  ─────────────────────────────────────",
        Style::default().fg(Color::Rgb(60, 60, 60)),
    ));

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  标题:      ", label),
            Span::styled(&todo.title, bold),
        ]),
        Line::from(""),
        divider.clone(),
        Line::from(""),
        Line::from(vec![
            Span::styled("  状态:      ", label),
            Span::styled(status_text, status_style),
        ]),
        Line::from(vec![
            Span::styled("  优先级:    ", label),
            Span::styled(priority_icon, priority_style),
        ]),
        tag_line,
        due_line,
        Line::from(""),
        divider,
        Line::from(""),
        Line::from(vec![
            Span::styled("  创建时间:  ", label),
            Span::styled(
                todo.created_at.replace('T', " "),
                Style::default().fg(Color::Rgb(80, 80, 80)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  [Esc] 关闭  [e] 编辑  [d] 删除  [Space] 切换完成",
                Style::default().fg(Color::Rgb(100, 100, 100)),
            ),
        ]),
    ];

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}
