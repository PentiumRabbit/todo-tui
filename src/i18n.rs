use crate::config::Lang;

pub struct T {
    pub lang: Lang,
}

impl T {
    pub fn new(lang: &Lang) -> Self {
        Self { lang: lang.clone() }
    }

    fn zh(&self) -> bool {
        self.lang == Lang::Zh
    }

    // --- filter panel ---
    pub fn filter_all(&self) -> &'static str {
        if self.zh() {
            "全部"
        } else {
            "All"
        }
    }
    pub fn filter_pending(&self) -> &'static str {
        if self.zh() {
            "未完成"
        } else {
            "Pending"
        }
    }
    pub fn filter_done(&self) -> &'static str {
        if self.zh() {
            "已完成"
        } else {
            "Done"
        }
    }
    pub fn filter_cancelled(&self) -> &'static str {
        if self.zh() {
            "已取消"
        } else {
            "Cancelled"
        }
    }
    pub fn filter_due_today(&self) -> &'static str {
        if self.zh() {
            "今日到期"
        } else {
            "Due Today"
        }
    }
    pub fn filter_overdue(&self) -> &'static str {
        if self.zh() {
            "已逾期"
        } else {
            "Overdue"
        }
    }
    pub fn panel_title(&self) -> &'static str {
        if self.zh() {
            " 过滤 "
        } else {
            " Filter "
        }
    }

    // --- status bar ---
    pub fn statusbar_normal(&self, _sort_label: &str) -> String {
        if self.zh() {
            " [a] 添加  [e] 编辑  [d] 删除  [/] 搜索  [s] 排序  [?] 帮助  [q] 退出".to_string()
        } else {
            " [a] Add  [e] Edit  [d] Del  [/] Search  [s] Sort  [?] Help  [q] Quit".to_string()
        }
    }
    pub fn statusbar_tag_panel(&self) -> &'static str {
        if self.zh() {
            " [l/→/Tab] 列表  [j/k] 移动  [g/G] 顶/底  [Enter] 选择  [q] 退出"
        } else {
            " [l/→/Tab] List  [j/k] Move  [g/G] Top/Bot  [Enter] Select  [q] Quit"
        }
    }
    pub fn statusbar_search(&self, query: &str) -> String {
        if self.zh() {
            format!(" 搜索: {}█  [Esc] 退出", query)
        } else {
            format!(" Search: {}█  [Esc] Exit", query)
        }
    }

    pub fn due_today_label(&self) -> String {
        if self.zh() {
            "⚠今天".to_string()
        } else {
            "⚠Today".to_string()
        }
    }

    // --- terminal too small ---
    pub fn terminal_too_small(&self) -> &'static str {
        if self.zh() {
            "终端太小，请调整至 80×24 以上"
        } else {
            "Terminal too small, please resize to 80×24 or larger"
        }
    }

    // --- delete confirm ---
    pub fn delete_confirm_title(&self) -> &'static str {
        if self.zh() {
            " 确认删除 "
        } else {
            " Confirm Delete "
        }
    }
    pub fn delete_confirm_msg(&self, title: &str) -> String {
        if self.zh() {
            format!("确定要删除 \"{}\" 吗？", title)
        } else {
            format!("Delete \"{}\"?", title)
        }
    }
    pub fn delete_confirm_hint(&self) -> (&'static str, &'static str) {
        if self.zh() {
            ("  [y/Enter] 确认  ", "[n/Esc] 取消")
        } else {
            ("  [y/Enter] Confirm  ", "[n/Esc] Cancel")
        }
    }

    // --- form ---
    pub fn form_add_title(&self) -> &'static str {
        if self.zh() {
            " 添加 Todo "
        } else {
            " Add Todo "
        }
    }
    pub fn form_edit_title(&self) -> &'static str {
        if self.zh() {
            " 编辑 Todo "
        } else {
            " Edit Todo "
        }
    }
    pub fn form_field_title(&self) -> &'static str {
        if self.zh() {
            "标题"
        } else {
            "Title"
        }
    }
    pub fn form_field_notes(&self) -> &'static str {
        if self.zh() {
            "备注"
        } else {
            "Notes"
        }
    }
    pub fn form_field_tags(&self) -> &'static str {
        if self.zh() {
            " 标签 (逗号/空格分隔，Enter确认) "
        } else {
            " Tags (comma/space separated, Enter to confirm) "
        }
    }
    pub fn form_field_priority(&self) -> &'static str {
        if self.zh() {
            "优先级"
        } else {
            "Priority"
        }
    }
    pub fn form_field_due_date(&self) -> &'static str {
        if self.zh() {
            "截止日期"
        } else {
            "Due Date"
        }
    }
    pub fn form_no_tags(&self) -> &'static str {
        if self.zh() {
            "无标签"
        } else {
            "no tags"
        }
    }
    pub fn form_hint(&self) -> Vec<(&'static str, &'static str)> {
        if self.zh() {
            vec![
                ("[Tab] ", "切换  "),
                ("[,/空格] ", "确认标签  "),
                ("[↑↓] ", "切换优先级  "),
                ("[Enter] ", "提交  "),
                ("[Esc] ", "取消"),
            ]
        } else {
            vec![
                ("[Tab] ", "Switch  "),
                ("[,/Space] ", "Confirm tag  "),
                ("[↑↓] ", "Priority  "),
                ("[Enter] ", "Submit  "),
                ("[Esc] ", "Cancel"),
            ]
        }
    }
    pub fn form_title_empty_error(&self) -> &'static str {
        if self.zh() {
            "标题不能为空"
        } else {
            "Title is required"
        }
    }

    // --- priority labels ---
    pub fn priority_high(&self) -> &'static str {
        if self.zh() {
            "高"
        } else {
            "High"
        }
    }
    pub fn priority_medium(&self) -> &'static str {
        if self.zh() {
            "中"
        } else {
            "Med"
        }
    }
    pub fn priority_low(&self) -> &'static str {
        if self.zh() {
            "低"
        } else {
            "Low"
        }
    }

    // --- detail popup ---
    pub fn detail_title(&self) -> &'static str {
        if self.zh() {
            " 详情 "
        } else {
            " Detail "
        }
    }
    pub fn detail_label_title(&self) -> &'static str {
        if self.zh() {
            "  标题:      "
        } else {
            "  Title:     "
        }
    }
    pub fn detail_label_status(&self) -> &'static str {
        if self.zh() {
            "  状态:      "
        } else {
            "  Status:    "
        }
    }
    pub fn detail_label_priority(&self) -> &'static str {
        if self.zh() {
            "  优先级:    "
        } else {
            "  Priority:  "
        }
    }
    pub fn detail_label_tags(&self) -> &'static str {
        if self.zh() {
            "  标签:      "
        } else {
            "  Tags:      "
        }
    }
    pub fn detail_label_due(&self) -> &'static str {
        if self.zh() {
            "  截止日期:  "
        } else {
            "  Due Date:  "
        }
    }
    pub fn detail_label_notes(&self) -> &'static str {
        if self.zh() {
            "  备注:      "
        } else {
            "  Notes:     "
        }
    }
    pub fn detail_label_created(&self) -> &'static str {
        if self.zh() {
            "  创建时间:  "
        } else {
            "  Created:   "
        }
    }
    pub fn detail_no_tags(&self) -> &'static str {
        if self.zh() {
            "无"
        } else {
            "none"
        }
    }
    pub fn detail_no_due(&self) -> &'static str {
        if self.zh() {
            "未设置"
        } else {
            "not set"
        }
    }
    pub fn detail_hint(&self) -> &'static str {
        if self.zh() {
            "  [Esc] 关闭  [e] 编辑  [d] 删除  [Space] 切换完成"
        } else {
            "  [Esc] Close  [e] Edit  [d] Delete  [Space] Toggle"
        }
    }
    pub fn status_pending(&self) -> &'static str {
        if self.zh() {
            "□ 未完成"
        } else {
            "□ Pending"
        }
    }
    pub fn status_pending_overdue(&self) -> &'static str {
        if self.zh() {
            "□ 未完成（已过期）"
        } else {
            "□ Pending (Overdue)"
        }
    }
    pub fn status_done(&self) -> &'static str {
        if self.zh() {
            "✓ 已完成"
        } else {
            "✓ Done"
        }
    }
    pub fn status_cancelled(&self) -> &'static str {
        if self.zh() {
            "✗ 已取消"
        } else {
            "✗ Cancelled"
        }
    }
    pub fn detail_overdue_suffix(&self) -> &'static str {
        if self.zh() {
            "  ⚠ 已过期"
        } else {
            "  ⚠ Overdue"
        }
    }
    pub fn detail_due_today_suffix(&self) -> &'static str {
        if self.zh() {
            "  ⚠ 今天到期"
        } else {
            "  ⚠ Due today"
        }
    }

    // --- help ---
    pub fn help_title(&self) -> &'static str {
        if self.zh() {
            " 快捷键帮助 "
        } else {
            " Keyboard Shortcuts "
        }
    }
    pub fn help_close_hint(&self) -> &'static str {
        if self.zh() {
            "  [?/Esc] 关闭"
        } else {
            "  [?/Esc] Close"
        }
    }
    pub fn help_section_nav(&self) -> &'static str {
        if self.zh() {
            "  导航"
        } else {
            "  Navigation"
        }
    }
    pub fn help_section_actions(&self) -> &'static str {
        if self.zh() {
            "  操作"
        } else {
            "  Actions"
        }
    }
    pub fn help_section_system(&self) -> &'static str {
        if self.zh() {
            "  系统"
        } else {
            "  System"
        }
    }
    pub fn help_entries(&self) -> Vec<(&'static str, &'static str)> {
        if self.zh() {
            vec![
                ("    j / ↓        ", "下移"),
                ("    k / ↑        ", "上移"),
                ("    g / Home     ", "跳到顶部"),
                ("    G / End      ", "跳到底部"),
                ("    h / ← / Tab  ", "切换到过滤栏"),
                ("    l / → / Tab  ", "切换到 Todo 列表"),
                ("    g / Home     ", "跳到顶部（过滤栏内）"),
                ("    G / End      ", "跳到底部（过滤栏内）"),
                ("    a            ", "添加 todo"),
                ("    e            ", "编辑选中 todo"),
                ("    d            ", "删除选中 todo"),
                ("    Enter        ", "查看详情"),
                ("    Space        ", "切换完成状态 (Pending ↔ Done)"),
                ("    x            ", "标记为已取消"),
                ("    s            ", "切换排序方式"),
                ("    /            ", "搜索（匹配标题/标签/备注）"),
                ("    L            ", "切换语言（中/英）"),
                ("    ?            ", "显示/关闭帮助"),
                ("    q            ", "退出"),
                ("    Esc          ", "取消/关闭弹窗"),
            ]
        } else {
            vec![
                ("    j / ↓        ", "Move down"),
                ("    k / ↑        ", "Move up"),
                ("    g / Home     ", "Jump to top"),
                ("    G / End      ", "Jump to bottom"),
                ("    h / ← / Tab  ", "Focus filter panel"),
                ("    l / → / Tab  ", "Focus todo list"),
                ("    g / Home     ", "Top (in filter panel)"),
                ("    G / End      ", "Bottom (in filter panel)"),
                ("    a            ", "Add todo"),
                ("    e            ", "Edit selected"),
                ("    d            ", "Delete selected"),
                ("    Enter        ", "View detail"),
                ("    Space        ", "Toggle done (Pending ↔ Done)"),
                ("    x            ", "Mark as cancelled"),
                ("    s            ", "Cycle sort order"),
                ("    /            ", "Search (title/tags/notes)"),
                ("    L            ", "Toggle language (En/Zh)"),
                ("    ?            ", "Show/hide help"),
                ("    q            ", "Quit"),
                ("    Esc          ", "Cancel / close popup"),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_all() {
        let t_en = T::new(&Lang::En);
        assert_eq!(t_en.filter_all(), "All");

        let t_zh = T::new(&Lang::Zh);
        assert_eq!(t_zh.filter_all(), "全部");
    }
}
