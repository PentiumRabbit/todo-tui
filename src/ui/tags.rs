use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{tag_color, AppState};

pub fn render_tag_panel(frame: &mut Frame, app: &AppState, area: Rect) {
    let is_focused = app.focus_tag_panel;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };

    let block = Block::default()
        .title(" 标签 ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);

    let items: Vec<ListItem> = app
        .tag_panel_items()
        .iter()
        .map(|item| {
            match item {
                None => {
                    // "全部" 条目
                    let is_selected = app.selected_tag.is_none();
                    let style = if is_selected {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(160, 160, 160))
                    };
                    ListItem::new(Line::from(vec![
                        Span::raw(" "),
                        Span::styled("◈ 全部", style),
                    ]))
                }
                Some(tag) => {
                    let (r, g, b) = tag_color(tag);
                    let is_selected = app.selected_tag.as_deref() == Some(tag);
                    let tag_style = if is_selected {
                        Style::default()
                            .fg(Color::Rgb(r, g, b))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(r, g, b))
                    };
                    // 截断超长标签名
                    let label = if tag.len() > 11 { &tag[..11] } else { tag };
                    ListItem::new(Line::from(vec![
                        Span::raw(" "),
                        Span::styled("# ", Style::default().fg(Color::Rgb(r, g, b))),
                        Span::styled(label, tag_style),
                    ]))
                }
            }
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.tag_panel_index));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 60, 80))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶");

    frame.render_stateful_widget(list, area, &mut state);
}
