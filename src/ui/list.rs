use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::{tag_color, AppState};
use crate::models::{AppMode, Priority, TodoStatus};

pub fn render_list(frame: &mut Frame, app: &AppState, area: Rect) {
    let is_active = !app.focus_tag_panel && matches!(app.mode, AppMode::Normal | AppMode::Search);
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };

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
        .map(|todo| {
            let (icon, icon_style) = match todo.status {
                TodoStatus::Pending if todo.is_overdue() => {
                    ("□", Style::default().fg(Color::Rgb(220, 100, 40)))
                }
                TodoStatus::Pending => ("□", Style::default().fg(Color::Rgb(200, 200, 200))),
                TodoStatus::Done => ("✓", Style::default().fg(Color::Rgb(80, 200, 120))),
                TodoStatus::Cancelled => ("✗", Style::default().fg(Color::Rgb(120, 120, 120))),
            };

            let title_style = match todo.status {
                TodoStatus::Done | TodoStatus::Cancelled => Style::default()
                    .fg(Color::Rgb(100, 100, 100))
                    .add_modifier(Modifier::CROSSED_OUT),
                TodoStatus::Pending if todo.is_overdue() => {
                    Style::default().fg(Color::Rgb(220, 100, 40))
                }
                _ => Style::default().fg(Color::Rgb(230, 230, 230)),
            };

            let (prio_icon, prio_style) = match todo.priority {
                Priority::High => ("▲", Style::default().fg(Color::Rgb(240, 80, 80))),
                Priority::Medium => ("●", Style::default().fg(Color::Rgb(220, 180, 60))),
                Priority::Low => ("▼", Style::default().fg(Color::Rgb(80, 180, 80))),
            };

            let dim = Style::default().fg(Color::Rgb(110, 110, 110));

            // 右侧：标签 + 截止日期，计算纯文本宽度
            let mut right_spans: Vec<Span> = Vec::new();
            let mut right_width: usize = 0;

            for tag in &todo.tags {
                let (r, g, b) = tag_color(tag);
                let text = format!("[{}] ", tag);
                right_width += text.as_str().width();
                right_spans.push(Span::styled(text, Style::default().fg(Color::Rgb(r, g, b))));
            }

            if let Some(d) = todo.due_date.as_deref() {
                let short = &d[5..]; // MM-DD
                let (text, style) = if todo.is_overdue() {
                    (
                        format!("⚠{}", short),
                        Style::default()
                            .fg(Color::Rgb(220, 100, 40))
                            .add_modifier(Modifier::BOLD),
                    )
                } else if todo.is_due_today() {
                    (
                        "⚠今天".to_string(),
                        Style::default().fg(Color::Rgb(220, 180, 60)),
                    )
                } else {
                    (short.to_string(), dim)
                };
                right_width += text.as_str().width();
                right_spans.push(Span::styled(text, style));
            }

            // 左侧固定部分：" □ ▲ " 各占1列，共5列，再加标题显示宽度
            let left_fixed = 1 + 1 + 1 + 1 + 1;
            let title_chars = todo.title.as_str().width();
            let left_width = left_fixed + title_chars;

            // 填充空格数，至少1个空格
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
        })
        .collect();

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.selected_index));
    }

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default()) // 无背景高亮
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}
