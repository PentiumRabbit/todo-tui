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
    let t = app.t();
    let title = if app.mode == AppMode::Add {
        t.form_add_title()
    } else {
        t.form_edit_title()
    };

    let popup = centered_rect(65, 20, area);
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
            Constraint::Length(3), // notes
            Constraint::Length(3), // tags
            Constraint::Length(3), // priority
            Constraint::Length(3), // due_date
            Constraint::Min(0),
            Constraint::Length(1), // hints
        ])
        .split(inner);

    render_text_field(
        frame,
        t.form_field_title(),
        &app.form.title,
        app.form.focused_field == FormField::Title,
        app.form.title_error.as_deref(),
        rows[1],
    );
    render_text_field(
        frame,
        t.form_field_notes(),
        &app.form.notes,
        app.form.focused_field == FormField::Notes,
        None,
        rows[2],
    );
    render_tags_field(frame, app, &t, rows[3]);
    render_priority_field(
        frame,
        t.form_field_priority(),
        priority_label(&app.form.priority, &t),
        &app.form.priority,
        app.form.focused_field == FormField::Priority,
        rows[4],
    );
    render_text_field(
        frame,
        t.form_field_due_date(),
        &app.form.due_date,
        app.form.focused_field == FormField::DueDate,
        app.form.due_date_error.as_deref(),
        rows[5],
    );

    frame.render_widget(Paragraph::new(hint_line(&t)), rows[7]);
}

fn render_tags_field(frame: &mut Frame, app: &AppState, t: &crate::i18n::T, area: Rect) {
    let focused = app.form.focused_field == FormField::Tags;
    let block = Block::default()
        .title(t.form_field_tags())
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
        spans.push(Span::styled(t.form_no_tags(), Style::default().fg(theme::FG_MUTED)));
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

fn render_priority_field(
    frame: &mut Frame,
    label: &str,
    value: &str,
    priority: &Priority,
    focused: bool,
    area: Rect,
) {
    use ratatui::style::Modifier;
    let color = match priority {
        Priority::High => theme::PRIORITY_HIGH,
        Priority::Medium => theme::PRIORITY_MEDIUM,
        Priority::Low => theme::PRIORITY_LOW,
    };
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
    let para = Paragraph::new(ratatui::text::Span::styled(
        display,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
    .block(block);
    frame.render_widget(para, area);
}

fn hint_line(t: &crate::i18n::T) -> Line<'static> {
    let entries = t.form_hint();
    let mut spans = vec![Span::raw("  ")];
    for (i, (key, desc)) in entries.iter().enumerate() {
        let key_style = if i == 3 {
            Style::default().fg(theme::ACTION_CONFIRM).add_modifier(Modifier::BOLD)
        } else if i == 4 {
            Style::default().fg(theme::ACTION_CANCEL)
        } else {
            Style::default().fg(theme::FG_LABEL)
        };
        spans.push(Span::styled(key.to_string(), key_style));
        spans.push(Span::raw(desc.to_string()));
    }
    Line::from(spans)
}

fn priority_label<'a>(p: &Priority, t: &'a crate::i18n::T) -> &'a str {
    match p {
        Priority::High => t.priority_high(),
        Priority::Medium => t.priority_medium(),
        Priority::Low => t.priority_low(),
    }
}
