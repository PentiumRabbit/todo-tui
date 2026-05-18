use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
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
        .border_type(theme::BORDER_TYPE)
        .border_style(border_style);

    // 内部可用宽度（减去左右 border 各1，再减去 highlight_symbol "▶ " 占2）
    let inner_width = area.width.saturating_sub(2 + 2) as usize;

    let due_today_label = app.t().due_today_label();
    let items: Vec<ListItem> = filtered
        .iter()
        .map(|todo| build_list_item(todo, inner_width, &due_today_label))
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

fn build_list_item(todo: &Todo, inner_width: usize, due_today_label: &str) -> ListItem<'static> {
    let (icon, icon_style) = status_icon(todo);
    let title_style = title_style(todo);

    let (right_spans, right_width) = build_right_spans(todo, due_today_label);

    let left_fixed = 2; // "□ "
    let title_chars = todo.title.as_str().width();
    let left_width = left_fixed + title_chars;

    // 计算 notes 可用宽度：title 和右侧之间至少留 1 空格分隔
    let (notes_span, notes_width) =
        if let Some(notes) = todo.notes.as_deref().filter(|n| !n.is_empty()) {
            let available = inner_width.saturating_sub(left_width + 2 + right_width);
            if available >= 4 {
                let prefix = "  "; // 两空格分隔
                let budget = available.saturating_sub(prefix.len());
                let truncated = truncate_to_width(notes, budget);
                let text = format!("{}{}", prefix, truncated);
                let w = text.as_str().width();
                (
                    Some(Span::styled(text, Style::default().fg(theme::FG_TEXT_DIM))),
                    w,
                )
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

    let pad = inner_width
        .saturating_sub(left_width + notes_width + right_width)
        .max(1);

    let mut spans = vec![
        Span::styled(icon, icon_style),
        Span::raw(" "),
        Span::styled(todo.title.clone(), title_style),
    ];
    if let Some(ns) = notes_span {
        spans.push(ns);
    }
    spans.push(Span::raw(" ".repeat(pad)));
    spans.extend(right_spans);

    ListItem::new(Line::from(spans))
}

fn status_icon(todo: &Todo) -> (&'static str, Style) {
    match todo.status {
        TodoStatus::Pending if todo.is_overdue() => {
            ("□", Style::default().fg(theme::STATUS_OVERDUE))
        }
        TodoStatus::Pending => {
            let color = match todo.priority {
                Priority::High => theme::PRIORITY_HIGH,
                Priority::Medium => theme::PRIORITY_MEDIUM,
                Priority::Low => theme::PRIORITY_LOW,
            };
            ("□", Style::default().fg(color))
        }
        TodoStatus::Done => ("✓", Style::default().fg(theme::STATUS_DONE)),
        TodoStatus::Cancelled => ("✗", Style::default().fg(theme::STATUS_CANCELLED)),
    }
}

fn title_style(todo: &Todo) -> Style {
    match todo.status {
        TodoStatus::Done | TodoStatus::Cancelled => theme::style_done(),
        TodoStatus::Pending if todo.is_overdue() => Style::default().fg(theme::STATUS_OVERDUE),
        TodoStatus::Pending => match todo.priority {
            Priority::High => Style::default().fg(theme::PRIORITY_HIGH),
            Priority::Medium => Style::default().fg(theme::PRIORITY_MEDIUM),
            Priority::Low => Style::default().fg(theme::PRIORITY_LOW),
        },
    }
}

fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut width = 0;
    let mut end = s.len();
    let ellipsis = "…";
    let ellipsis_w = ellipsis.width();

    for (i, ch) in s.char_indices() {
        let cw = ch.to_string().as_str().width();
        if width + cw > max_width {
            // try to fit ellipsis
            let mut trunc_w = 0;
            let mut trunc_end = 0;
            for (j, c) in s.char_indices() {
                let cw2 = c.to_string().as_str().width();
                if trunc_w + cw2 + ellipsis_w > max_width {
                    end = trunc_end;
                    return format!("{}{}", &s[..end], ellipsis);
                }
                trunc_w += cw2;
                trunc_end = j + c.len_utf8();
            }
            end = i;
            return format!("{}{}", &s[..end], ellipsis);
        }
        width += cw;
        end = i + ch.len_utf8();
    }
    s[..end].to_string()
}

fn build_right_spans(todo: &Todo, due_today_label: &str) -> (Vec<Span<'static>>, usize) {
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
                due_today_label.to_string(),
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
