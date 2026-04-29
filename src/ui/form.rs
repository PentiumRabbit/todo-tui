use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::app::{AppState, tag_color};
use crate::models::{AppMode, FormField, Priority};
use crate::ui::centered_rect;

pub fn render_form(frame: &mut Frame, app: &AppState, area: Rect) {
    let is_add = app.mode == AppMode::Add;
    let title = if is_add { " 添加 Todo " } else { " 编辑 Todo " };

    let popup = centered_rect(65, 16, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // padding
            Constraint::Length(3), // title
            Constraint::Length(3), // tags
            Constraint::Length(3), // priority
            Constraint::Length(3), // due_date
            Constraint::Min(0),
            Constraint::Length(1), // hints
        ])
        .split(inner);

    render_field(
        frame, "标题", &app.form.title,
        app.form.focused_field == FormField::Title,
        app.form.title_error.as_deref(), rows[1],
    );
    render_tags_field(frame, app, rows[2]);
    render_select_field(
        frame, "优先级", priority_label(&app.form.priority),
        app.form.focused_field == FormField::Priority, rows[3],
    );
    render_field(
        frame, "截止日期", &app.form.due_date,
        app.form.focused_field == FormField::DueDate,
        app.form.due_date_error.as_deref(), rows[4],
    );

    let hints = Line::from(vec![
        Span::styled("  [Tab] ", Style::default().fg(Color::Rgb(120,120,120))),
        Span::raw("切换  "),
        Span::styled("[,/空格] ", Style::default().fg(Color::Rgb(120,120,120))),
        Span::raw("确认标签  "),
        Span::styled("[↑↓] ", Style::default().fg(Color::Rgb(120,120,120))),
        Span::raw("切换优先级  "),
        Span::styled("[Enter] ", Style::default().fg(Color::Rgb(80,200,120)).add_modifier(Modifier::BOLD)),
        Span::raw("提交  "),
        Span::styled("[Esc] ", Style::default().fg(Color::Rgb(200,80,80))),
        Span::raw("取消"),
    ]);
    frame.render_widget(Paragraph::new(hints), rows[6]);
}

fn render_tags_field(frame: &mut Frame, app: &AppState, area: Rect) {
    let focused = app.form.focused_field == FormField::Tags;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };

    let block = Block::default()
        .title(" 标签 (逗号/空格分隔，Enter确认) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // 已确认标签 + 正在输入的内容
    let mut spans: Vec<Span> = Vec::new();
    for tag in &app.form.tags {
        let (r, g, b) = tag_color(tag);
        spans.push(Span::styled(
            format!("[{}] ", tag),
            Style::default().fg(Color::Rgb(r, g, b)).add_modifier(Modifier::BOLD),
        ));
    }
    if focused {
        spans.push(Span::styled(
            format!("{}█", app.form.tag_input),
            Style::default().fg(Color::White),
        ));
    } else if app.form.tags.is_empty() {
        spans.push(Span::styled("无标签", Style::default().fg(Color::Rgb(80, 80, 80))));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), inner);
}

fn render_field(frame: &mut Frame, label: &str, value: &str, focused: bool, error: Option<&str>, area: Rect) {
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };

    let title = if let Some(err) = error {
        format!(" {} — {} ", label, err)
    } else {
        format!(" {} ", label)
    };
    let title_style = if error.is_some() {
        Style::default().fg(Color::Rgb(220, 80, 80))
    } else {
        Style::default()
    };

    let display = if focused { format!("{}█", value) } else { value.to_string() };

    let block = Block::default()
        .title(Span::styled(title, title_style))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    frame.render_widget(Paragraph::new(display).block(block), area);
}

fn render_select_field(frame: &mut Frame, label: &str, value: &str, focused: bool, area: Rect) {
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };
    let display = if focused { format!("◀ {} ▶", value) } else { format!("  {}", value) };
    let block = Block::default()
        .title(format!(" {} ", label))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    frame.render_widget(Paragraph::new(display).block(block), area);
}

fn priority_label(p: &Priority) -> &'static str {
    match p {
        Priority::High => "高",
        Priority::Medium => "中",
        Priority::Low => "低",
    }
}
