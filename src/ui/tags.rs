use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{tag_color, AppState, PanelItem};
use crate::ui::theme;

/// 渲染左侧标签侧边栏。
pub fn render_tag_panel(frame: &mut Frame, app: &AppState, area: Rect) {
    let t = app.t();
    let block = Block::default()
        .title(t.panel_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_for_focus(app.focus_tag_panel));

    let items: Vec<ListItem> = app
        .tag_panel_items()
        .iter()
        .map(|item| build_item(item, app, &t))
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.tag_panel_index));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(theme::BG_TAG_HIGHLIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶");

    frame.render_stateful_widget(list, area, &mut state);
}

fn build_item(item: &PanelItem, app: &AppState, t: &crate::i18n::T) -> ListItem<'static> {
    let is_selected = item.to_filter() == app.filter;

    if item.is_builtin() {
        builtin_item(item, is_selected, t)
    } else {
        tag_item(item, is_selected, t)
    }
}

fn builtin_item(item: &PanelItem, is_selected: bool, t: &crate::i18n::T) -> ListItem<'static> {
    let (icon, color) = match item {
        PanelItem::All => ("◈", Color::White),
        PanelItem::Status(crate::models::TodoStatus::Pending) => ("□", theme::STATUS_PENDING),
        PanelItem::Status(crate::models::TodoStatus::Done) => ("✓", theme::STATUS_DONE),
        PanelItem::Status(crate::models::TodoStatus::Cancelled) => ("✗", theme::STATUS_CANCELLED),
        PanelItem::DueToday => ("⚑", theme::STATUS_DUE_TODAY),
        PanelItem::Overdue => ("⚠", theme::STATUS_OVERDUE),
        PanelItem::Tag(_) => unreachable!(),
    };

    let label = item.label(t).to_string();
    let style = if is_selected {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::FG_TAG_INACTIVE)
    };

    ListItem::new(Line::from(vec![
        Span::raw(" "),
        Span::styled(format!("{} {}", icon, label), style),
    ]))
}

fn tag_item(item: &PanelItem, is_selected: bool, t: &crate::i18n::T) -> ListItem<'static> {
    let tag = item.label(t);
    let (r, g, b) = tag_color(tag);
    let tag_style = if is_selected {
        Style::default()
            .fg(Color::Rgb(r, g, b))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(r, g, b))
    };
    let label = if tag.len() > 11 { &tag[..11] } else { tag };
    ListItem::new(Line::from(vec![
        Span::raw(" "),
        Span::styled("# ", Style::default().fg(Color::Rgb(r, g, b))),
        Span::styled(label.to_string(), tag_style),
    ]))
}
