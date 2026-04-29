use ratatui::style::{Color, Modifier, Style};

// ── 边框 ─────────────────────────────────────────
pub const BORDER_ACTIVE: Color = Color::Cyan;
pub const BORDER_INACTIVE: Color = Color::Rgb(80, 80, 80);

// ── 背景 ─────────────────────────────────────────
pub const BG_STATUSBAR: Color = Color::Rgb(40, 40, 40);
pub const BG_TAG_HIGHLIGHT: Color = Color::Rgb(40, 60, 80);

// ── 文字基础 ──────────────────────────────────────
pub const FG_STATUSBAR: Color = Color::Rgb(180, 180, 180);
pub const FG_TEXT_DIM: Color = Color::Rgb(110, 110, 110);
pub const FG_LABEL: Color = Color::Rgb(120, 120, 120);
pub const FG_VALUE: Color = Color::Rgb(220, 220, 220);
pub const FG_MUTED: Color = Color::Rgb(80, 80, 80);
pub const FG_HINT: Color = Color::Rgb(100, 100, 100);
pub const FG_TAG_INACTIVE: Color = Color::Rgb(160, 160, 160);
pub const FG_DIVIDER: Color = Color::Rgb(60, 60, 60);

// ── 状态颜色 ──────────────────────────────────────
pub const STATUS_DONE: Color = Color::Rgb(80, 200, 120);
pub const STATUS_CANCELLED: Color = Color::Rgb(120, 120, 120);
pub const STATUS_OVERDUE: Color = Color::Rgb(220, 100, 40);
pub const STATUS_DUE_TODAY: Color = Color::Rgb(220, 180, 60);
pub const STATUS_PENDING: Color = Color::Rgb(200, 200, 200);

// ── 优先级颜色 ────────────────────────────────────
pub const PRIORITY_HIGH: Color = Color::Rgb(200, 80, 80);
pub const PRIORITY_MEDIUM: Color = Color::Rgb(190, 150, 50);
pub const PRIORITY_LOW: Color = Color::Rgb(80, 140, 200);

// ── 操作颜色 ──────────────────────────────────────
pub const ACTION_CONFIRM: Color = Color::Rgb(80, 200, 120);
pub const ACTION_CANCEL: Color = Color::Rgb(200, 80, 80);
pub const ACTION_ERROR: Color = Color::Rgb(220, 80, 80);

// ── 快捷 Style 工厂 ───────────────────────────────

/// 激活边框样式。
pub fn border_active() -> Style {
    Style::default().fg(BORDER_ACTIVE)
}

/// 非激活边框样式。
pub fn border_inactive() -> Style {
    Style::default().fg(BORDER_INACTIVE)
}

/// 根据焦点状态返回边框样式。
pub fn border_for_focus(focused: bool) -> Style {
    if focused {
        border_active()
    } else {
        border_inactive()
    }
}

/// 已完成条目的删除线样式。
pub fn style_done() -> Style {
    Style::default()
        .fg(FG_MUTED)
        .add_modifier(Modifier::CROSSED_OUT)
}

/// 过期条目样式（加粗）。
pub fn style_overdue_bold() -> Style {
    Style::default()
        .fg(STATUS_OVERDUE)
        .add_modifier(Modifier::BOLD)
}

/// 今天到期样式（加粗）。
pub fn style_due_today_bold() -> Style {
    Style::default()
        .fg(STATUS_DUE_TODAY)
        .add_modifier(Modifier::BOLD)
}
