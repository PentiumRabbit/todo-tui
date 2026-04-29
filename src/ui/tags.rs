use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{tag_color, AppState};
use crate::ui::theme;

/// 渲染左侧标签侧边栏。
pub fn render_tag_panel(frame: &mut Frame, app: &AppState, area: Rect) {
    let block = Block::default()
        .title(" 标签 ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::border_for_focus(app.focus_tag_panel));

    let items: Vec<ListItem> = app
        .tag_panel_items()
        .iter()
        .map(|item| match item {
            None => all_item(app),
            Some(tag) => tag_item(tag, app),
        })
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

fn all_item(app: &AppState) -> ListItem<'static> {
    let is_selected = app.selected_tag.is_none();
    let style = if is_selected {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::FG_TAG_INACTIVE)
    };
    ListItem::new(Line::from(vec![
        Span::raw(" "),
        Span::styled("◈ 全部", style),
    ]))
}

fn tag_item(tag: &str, app: &AppState) -> ListItem<'static> {
    let (r, g, b) = tag_color(tag);
    let is_selected = app.selected_tag.as_deref() == Some(tag);
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
