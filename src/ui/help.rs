use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::centered_rect;

pub fn render_help(frame: &mut Frame, area: Rect) {
    let popup = centered_rect(55, 20, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(" 快捷键帮助 ")
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

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("  导航", header)),
        Line::from(vec![
            Span::styled("    j / ↓     ", key),
            Span::styled("下移", desc),
        ]),
        Line::from(vec![
            Span::styled("    k / ↑     ", key),
            Span::styled("上移", desc),
        ]),
        Line::from(vec![
            Span::styled("    g / Home  ", key),
            Span::styled("跳到顶部", desc),
        ]),
        Line::from(vec![
            Span::styled("    G / End   ", key),
            Span::styled("跳到底部", desc),
        ]),
        Line::from(""),
        Line::from(Span::styled("  操作", header)),
        Line::from(vec![
            Span::styled("    a         ", key),
            Span::styled("添加 todo", desc),
        ]),
        Line::from(vec![
            Span::styled("    e         ", key),
            Span::styled("编辑选中 todo", desc),
        ]),
        Line::from(vec![
            Span::styled("    d         ", key),
            Span::styled("删除选中 todo", desc),
        ]),
        Line::from(vec![
            Span::styled("    Space     ", key),
            Span::styled("切换完成状态", desc),
        ]),
        Line::from(vec![
            Span::styled("    /         ", key),
            Span::styled("搜索", desc),
        ]),
        Line::from(""),
        Line::from(Span::styled("  系统", header)),
        Line::from(vec![
            Span::styled("    ?         ", key),
            Span::styled("显示/关闭帮助", desc),
        ]),
        Line::from(vec![
            Span::styled("    q         ", key),
            Span::styled("退出", desc),
        ]),
        Line::from(vec![
            Span::styled("    Esc       ", key),
            Span::styled("取消/关闭弹窗", desc),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  [?/Esc] 关闭",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(lines), inner);
}
