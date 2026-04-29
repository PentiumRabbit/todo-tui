use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{tag_color, AppState};
use crate::models::{AppMode, FormField, Priority};
use crate::ui::{centered_rect, theme};

/// 渲染添加/编辑 Todo 表单弹窗。
pub fn render_form(frame: &mut Frame, app: &AppState, area: Rect) {
    let title = if app.mode == AppMode::Add {
        " 添加 Todo "
    } else {
        " 编辑 Todo "
    };

    let popup = centered_rect(65, 16, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_active());

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

    render_text_field(
        frame,
        "标题",
        &app.form.title,
        app.form.focused_field == FormField::Title,
        app.form.title_error.as_deref(),
        rows[1],
    );
    render_tags_field(frame, app, rows[2]);
    render_select_field(
        frame,
        "优先级",
        priority_label(&app.form.priority),
        app.form.focused_field == FormField::Priority,
        rows[3],
    );
    render_text_field(
        frame,
        "截止日期",
        &app.form.due_date,
        app.form.focused_field == FormField::DueDate,
        app.form.due_date_error.as_deref(),
        rows[4],
    );

    frame.render_widget(Paragraph::new(hint_line()), rows[6]);
}

fn render_tags_field(frame: &mut Frame, app: &AppState, area: Rect) {
    let focused = app.form.focused_field == FormField::Tags;
    let block = Block::default()
        .title(" 标签 (逗号/空格分隔，Enter确认) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_for_focus(focused));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut spans: Vec<Span> = Vec::new();
    for tag in &app.form.tags {
        let (r, g, b) = tag_color(tag);
        spans.push(Span::styled(
            format!("[{}] ", tag),
            Style::default()
                .fg(Color::Rgb(r, g, b))
                .add_modifier(Modifier::BOLD),
        ));
    }
    if focused {
        spans.push(Span::styled(
            format!("{}█", app.form.tag_input),
            Style::default().fg(Color::White),
        ));
    } else if app.form.tags.is_empty() {
        spans.push(Span::styled("无标签", Style::default().fg(theme::FG_MUTED)));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), inner);
}

fn render_text_field(
    frame: &mut Frame,
    label: &str,
    value: &str,
    focused: bool,
    error: Option<&str>,
    area: Rect,
) {
    let title = match error {
        Some(err) => format!(" {} — {} ", label, err),
        None => format!(" {} ", label),
    };
    let title_style = if error.is_some() {
        Style::default().fg(theme::ACTION_ERROR)
    } else {
        Style::default()
    };

    let display = if focused {
        format!("{}█", value)
    } else {
        value.to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, title_style))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_for_focus(focused));

    frame.render_widget(Paragraph::new(display).block(block), area);
}

fn render_select_field(frame: &mut Frame, label: &str, value: &str, focused: bool, area: Rect) {
    let display = if focused {
        format!("◀ {} ▶", value)
    } else {
        format!("  {}", value)
    };
    let block = Block::default()
        .title(format!(" {} ", label))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_for_focus(focused));
    frame.render_widget(Paragraph::new(display).block(block), area);
}

fn hint_line() -> Line<'static> {
    Line::from(vec![
        Span::styled("  [Tab] ", Style::default().fg(theme::FG_LABEL)),
        Span::raw("切换  "),
        Span::styled("[,/空格] ", Style::default().fg(theme::FG_LABEL)),
        Span::raw("确认标签  "),
        Span::styled("[↑↓] ", Style::default().fg(theme::FG_LABEL)),
        Span::raw("切换优先级  "),
        Span::styled(
            "[Enter] ",
            Style::default()
                .fg(theme::ACTION_CONFIRM)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("提交  "),
        Span::styled("[Esc] ", Style::default().fg(theme::ACTION_CANCEL)),
        Span::raw("取消"),
    ])
}

fn priority_label(p: &Priority) -> &'static str {
    match p {
        Priority::High => "高",
        Priority::Medium => "中",
        Priority::Low => "低",
    }
}
