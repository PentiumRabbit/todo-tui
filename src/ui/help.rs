use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::ui::centered_rect;

pub fn render_help(frame: &mut Frame, app: &AppState, area: Rect) {
    let t = app.t();
    let popup = centered_rect(58, 28, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(t.help_title())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let header = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let key = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let desc = Style::default().fg(Color::White);

    let entries = t.help_entries();
    // nav: 0..8, actions: 8..16, system: 16..
    let nav_entries = &entries[..8];
    let action_entries = &entries[8..16];
    let system_entries = &entries[16..];

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(t.help_section_nav(), header)),
    ];
    for (k, d) in nav_entries {
        lines.push(Line::from(vec![
            Span::styled(k.to_string(), key),
            Span::styled(d.to_string(), desc),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(t.help_section_actions(), header)));
    for (k, d) in action_entries {
        lines.push(Line::from(vec![
            Span::styled(k.to_string(), key),
            Span::styled(d.to_string(), desc),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(t.help_section_system(), header)));
    for (k, d) in system_entries {
        lines.push(Line::from(vec![
            Span::styled(k.to_string(), key),
            Span::styled(d.to_string(), desc),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        t.help_close_hint(),
        Style::default().fg(Color::DarkGray),
    )));

    frame.render_widget(Paragraph::new(lines), inner);
}
