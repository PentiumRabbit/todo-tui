use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::{tag_color, AppState};
use crate::models::{AppMode, Priority, Todo, TodoStatus};
use crate::ui::theme;

/// 渲染 Todo 列表面板。
pub fn render_list(frame: &mut Frame, app: &AppState, area: Rect) {
    let is_active = !app.focus_tag_panel && matches!(app.mode, AppMode::Normal | AppMode::Search);
    let border_style = theme::border_for_focus(is_active);

    let filtered = app.filtered_todos();
    let total = filtered.len();
    let idx = if total == 0 {
        0
    } else {
        app.selected_index + 1
    };
    let title = format!(" Todo ({}/{}) ", idx, total);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    // 内部可用宽度（减去左右 border 各1，再减去 highlight_symbol "▶ " 占2）
    let inner_width = area.width.saturating_sub(2 + 2) as usize;

    let items: Vec<ListItem> = filtered
        .iter()
        .map(|todo| build_list_item(todo, inner_width))
        .collect();

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.selected_index));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn build_list_item(todo: &Todo, inner_width: usize) -> ListItem<'static> {
    let (icon, icon_style) = status_icon(todo);
    let (prio_icon, prio_style) = priority_icon(&todo.priority);
    let title_style = title_style(todo);

    let (right_spans, right_width) = build_right_spans(todo);

    let left_fixed = 5; // " □ ▲ "
    let title_chars = todo.title.as_str().width();
    let left_width = left_fixed + title_chars;

    let pad = if left_width + right_width + 1 < inner_width {
        inner_width - left_width - right_width
    } else {
        1
    };

    let mut spans = vec![
        Span::raw(" "),
        Span::styled(icon, icon_style),
        Span::raw(" "),
        Span::styled(prio_icon, prio_style),
        Span::raw(" "),
        Span::styled(todo.title.clone(), title_style),
        Span::raw(" ".repeat(pad)),
    ];
    spans.extend(right_spans);

    ListItem::new(Line::from(spans))
}

fn status_icon(todo: &Todo) -> (&'static str, Style) {
    match todo.status {
        TodoStatus::Pending if todo.is_overdue() => {
            ("□", Style::default().fg(theme::STATUS_OVERDUE))
        }
        TodoStatus::Pending => ("□", Style::default().fg(theme::STATUS_PENDING)),
        TodoStatus::Done => ("✓", Style::default().fg(theme::STATUS_DONE)),
        TodoStatus::Cancelled => ("✗", Style::default().fg(theme::STATUS_CANCELLED)),
    }
}

fn priority_icon(priority: &Priority) -> (&'static str, Style) {
    match priority {
        Priority::High => ("▲", Style::default().fg(theme::PRIORITY_HIGH)),
        Priority::Medium => ("●", Style::default().fg(theme::PRIORITY_MEDIUM)),
        Priority::Low => ("▼", Style::default().fg(theme::PRIORITY_LOW)),
    }
}

fn title_style(todo: &Todo) -> Style {
    match todo.status {
        TodoStatus::Done | TodoStatus::Cancelled => theme::style_done(),
        TodoStatus::Pending if todo.is_overdue() => Style::default().fg(theme::STATUS_OVERDUE),
        _ => Style::default().fg(theme::FG_TEXT),
    }
}

fn build_right_spans(todo: &Todo) -> (Vec<Span<'static>>, usize) {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut width: usize = 0;

    for tag in &todo.tags {
        let (r, g, b) = tag_color(tag);
        let text = format!("[{}] ", tag);
        width += text.as_str().width();
        spans.push(Span::styled(
            text,
            Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
        ));
    }

    if let Some(d) = todo.due_date.as_deref() {
        let short = d[5..].to_string(); // MM-DD
        let (text, style) = if todo.is_overdue() {
            (format!("⚠{}", short), theme::style_overdue_bold())
        } else if todo.is_due_today() {
            (
                "⚠今天".to_string(),
                Style::default().fg(theme::STATUS_DUE_TODAY),
            )
        } else {
            (short, Style::default().fg(theme::FG_TEXT_DIM))
        };
        width += text.as_str().width();
        spans.push(Span::styled(text, style));
    }

    (spans, width)
}
