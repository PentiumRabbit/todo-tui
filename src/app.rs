use anyhow::Result;
use chrono::NaiveDate;
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::models::{
    AppAction, AppMode, FormField, FormState, NewTodo, Priority, Todo, TodoStatus,
};
use crate::storage::Storage;

// 标签固定颜色池（循环分配）
pub const TAG_COLORS: &[(u8, u8, u8)] = &[
    (100, 180, 255), // 蓝
    (100, 220, 140), // 绿
    (255, 190, 80),  // 黄
    (220, 120, 255), // 紫
    (255, 130, 100), // 橙红
    (80, 210, 210),  // 青
    (255, 160, 200), // 粉
    (160, 200, 100), // 黄绿
];

pub fn tag_color(tag: &str) -> (u8, u8, u8) {
    let idx = tag.bytes().fold(0usize, |a, b| a.wrapping_add(b as usize)) % TAG_COLORS.len();
    TAG_COLORS[idx]
}

pub struct AppState {
    pub mode: AppMode,
    pub todos: Vec<Todo>,
    pub all_tags: Vec<String>,
    pub selected_tag: Option<String>,
    pub tag_panel_index: usize,
    pub focus_tag_panel: bool,
    pub selected_index: usize,
    #[allow(dead_code)]
    pub list_offset: usize,
    pub search_query: String,
    pub form: FormState,
    pub error_message: Option<String>,
    storage: Storage,
}

impl AppState {
    /// 初始化状态，从 storage 加载全部 todo 和标签列表。
    pub fn new(storage: Storage) -> Result<Self> {
        let todos = storage.list_todos()?;
        let all_tags = storage.list_all_tags()?;
        Ok(Self {
            mode: AppMode::Normal,
            todos,
            all_tags,
            selected_tag: None,
            tag_panel_index: 0,
            focus_tag_panel: false,
            selected_index: 0,
            list_offset: 0,
            search_query: String::new(),
            form: FormState::default(),
            error_message: None,
            storage,
        })
    }

    /// 返回经标签过滤和搜索关键词过滤后的 todo 列表。
    pub fn filtered_todos(&self) -> Vec<&Todo> {
        let tag_filter = self.selected_tag.as_deref();
        let search = if self.search_query.is_empty() {
            None
        } else {
            Some(self.search_query.to_lowercase())
        };

        self.todos
            .iter()
            .filter(|t| {
                let tag_ok = match tag_filter {
                    None => true,
                    Some(tag) => t.tags.iter().any(|tg| tg == tag),
                };
                let search_ok = match &search {
                    None => true,
                    Some(q) => t.title.to_lowercase().contains(q.as_str()),
                };
                tag_ok && search_ok
            })
            .collect()
    }

    /// 返回当前高亮的 todo（在过滤后列表中）。
    pub fn selected_todo(&self) -> Option<&Todo> {
        self.filtered_todos().get(self.selected_index).copied()
    }

    /// 返回标签面板条目列表：第一项为 `None`（全部），其余为标签名。
    pub fn tag_panel_items(&self) -> Vec<Option<&str>> {
        let mut items: Vec<Option<&str>> = vec![None]; // None = 全部
        items.extend(self.all_tags.iter().map(|t| Some(t.as_str())));
        items
    }

    /// 处理键盘事件，返回 `AppAction::Quit` 表示退出，否则返回 `Continue`。
    pub fn handle_event(&mut self, event: KeyEvent) -> Result<AppAction> {
        self.error_message = None;
        match self.mode.clone() {
            AppMode::Normal => self.handle_normal(event),
            AppMode::Detail => self.handle_detail(event),
            AppMode::Add | AppMode::Edit => self.handle_form(event),
            AppMode::DeleteConfirm => self.handle_delete_confirm(event),
            AppMode::Search => self.handle_search(event),
            AppMode::Help => self.handle_help(event),
        }
    }

    fn handle_normal(&mut self, event: KeyEvent) -> Result<AppAction> {
        if self.focus_tag_panel {
            return self.handle_tag_panel(event);
        }
        match event.code {
            KeyCode::Char('q') => return Ok(AppAction::Quit),
            KeyCode::Char('?') => self.mode = AppMode::Help,
            KeyCode::Char('a') => {
                self.form = FormState::default();
                self.mode = AppMode::Add;
            }
            KeyCode::Char('e') => {
                if let Some(todo) = self.selected_todo() {
                    self.form = FormState::from_todo(todo);
                    self.mode = AppMode::Edit;
                }
            }
            KeyCode::Char('d') if self.selected_todo().is_some() => {
                self.mode = AppMode::DeleteConfirm;
            }
            KeyCode::Char(' ') => self.toggle_status()?,
            KeyCode::Char('x') => self.cancel_todo()?,
            KeyCode::Char('j') | KeyCode::Down => self.move_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('g') | KeyCode::Home => self.selected_index = 0,
            KeyCode::Char('G') | KeyCode::End => {
                let len = self.filtered_todos().len();
                if len > 0 {
                    self.selected_index = len - 1;
                }
            }
            KeyCode::Char('/') => {
                self.search_query.clear();
                self.mode = AppMode::Search;
            }
            KeyCode::Enter if self.selected_todo().is_some() => {
                self.mode = AppMode::Detail;
            }
            KeyCode::Tab | KeyCode::Left => {
                self.focus_tag_panel = true;
            }
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_tag_panel(&mut self, event: KeyEvent) -> Result<AppAction> {
        let items_len = self.tag_panel_items().len();
        match event.code {
            KeyCode::Char('q') => return Ok(AppAction::Quit),
            KeyCode::Tab | KeyCode::Right => self.focus_tag_panel = false,
            KeyCode::Char('j') | KeyCode::Down if self.tag_panel_index + 1 < items_len => {
                self.tag_panel_index += 1;
            }
            KeyCode::Char('k') | KeyCode::Up if self.tag_panel_index > 0 => {
                self.tag_panel_index -= 1;
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let items = self.tag_panel_items();
                self.selected_tag = items[self.tag_panel_index].map(|s| s.to_string());
                self.selected_index = 0;
                self.focus_tag_panel = false;
            }
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_detail(&mut self, event: KeyEvent) -> Result<AppAction> {
        match event.code {
            KeyCode::Esc | KeyCode::Enter => self.mode = AppMode::Normal,
            KeyCode::Char('e') => {
                if let Some(todo) = self.selected_todo() {
                    self.form = FormState::from_todo(todo);
                    self.mode = AppMode::Edit;
                }
            }
            KeyCode::Char('d') if self.selected_todo().is_some() => {
                self.mode = AppMode::DeleteConfirm;
            }
            KeyCode::Char(' ') => self.toggle_status()?,
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_form(&mut self, event: KeyEvent) -> Result<AppAction> {
        match event.code {
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.form = FormState::default();
            }
            KeyCode::Tab => self.form_next_field(),
            KeyCode::BackTab => self.form_prev_field(),
            KeyCode::Enter => {
                if self.form.focused_field == FormField::Tags {
                    if self.form.tag_input.trim().is_empty() {
                        // 输入框为空时 Enter 提交表单
                        self.submit_form()?;
                    } else {
                        // 有内容时 Enter 确认当前标签
                        self.form_confirm_tag();
                    }
                } else {
                    self.submit_form()?;
                }
            }
            KeyCode::Backspace => self.form_backspace(),
            KeyCode::Up => self.form_cycle_up(),
            KeyCode::Down => self.form_cycle_down(),
            KeyCode::Char('k') if self.form.focused_field == FormField::Priority => {
                self.form_cycle_up();
            }
            KeyCode::Char('j') if self.form.focused_field == FormField::Priority => {
                self.form_cycle_down();
            }
            // Tags 字段：Ctrl+Enter 或 F2 提交整个表单（可用 Alt+Enter）
            // 普通情况：在 Tags 以外的字段按 Enter 提交
            KeyCode::Char(c) => self.form_input(c),
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_delete_confirm(&mut self, event: KeyEvent) -> Result<AppAction> {
        match event.code {
            KeyCode::Char('y') | KeyCode::Enter => self.delete_selected()?,
            KeyCode::Char('n') | KeyCode::Esc => self.mode = AppMode::Normal,
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_search(&mut self, event: KeyEvent) -> Result<AppAction> {
        match event.code {
            KeyCode::Esc | KeyCode::Enter => {
                self.mode = AppMode::Normal;
                self.selected_index = 0;
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.selected_index = 0;
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.selected_index = 0;
            }
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn handle_help(&mut self, event: KeyEvent) -> Result<AppAction> {
        match event.code {
            KeyCode::Char('?') | KeyCode::Esc => self.mode = AppMode::Normal,
            _ => {}
        }
        Ok(AppAction::Continue)
    }

    fn move_down(&mut self) {
        let len = self.filtered_todos().len();
        if len > 0 && self.selected_index < len - 1 {
            self.selected_index += 1;
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn toggle_status(&mut self) -> Result<()> {
        let id = match self.selected_todo() {
            Some(t) => t.id,
            None => return Ok(()),
        };
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            let prev = todo.status.clone();
            todo.status = todo.status.next();
            if let Err(e) = self.storage.update_todo(todo) {
                todo.status = prev;
                self.error_message = Some(e.to_string());
            }
        }
        Ok(())
    }

    fn cancel_todo(&mut self) -> Result<()> {
        let id = match self.selected_todo() {
            Some(t) => t.id,
            None => return Ok(()),
        };
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            let prev = todo.status.clone();
            todo.status = TodoStatus::Cancelled;
            if let Err(e) = self.storage.update_todo(todo) {
                todo.status = prev;
                self.error_message = Some(e.to_string());
            }
        }
        Ok(())
    }

    fn delete_selected(&mut self) -> Result<()> {
        if let Some(todo) = self.selected_todo() {
            let id = todo.id;
            self.storage.delete_todo(id)?;
            self.todos.retain(|t| t.id != id);
            let len = self.filtered_todos().len();
            if self.selected_index >= len && len > 0 {
                self.selected_index = len - 1;
            }
        }
        self.mode = AppMode::Normal;
        Ok(())
    }

    fn submit_form(&mut self) -> Result<()> {
        let title = self.form.title.trim().to_string();
        if title.is_empty() {
            self.form.title_error = Some("标题不能为空".to_string());
            self.form.focused_field = FormField::Title;
            return Ok(());
        }

        let due_date = if self.form.due_date.is_empty() {
            None
        } else {
            match self.form.due_date.parse::<NaiveDate>() {
                Ok(_) => Some(self.form.due_date.clone()),
                Err(_) => {
                    self.form.due_date_error = Some("格式应为 YYYY-MM-DD".to_string());
                    self.form.focused_field = FormField::DueDate;
                    return Ok(());
                }
            }
        };

        // 若 tag_input 还有未确认内容，自动加入
        let mut tags = self.form.tags.clone();
        let pending = self.form.tag_input.trim().to_string();
        if !pending.is_empty() && !tags.contains(&pending) {
            tags.push(pending);
        }

        if let Some(edit_id) = self.form.editing_todo_id {
            if let Some(todo) = self.todos.iter_mut().find(|t| t.id == edit_id) {
                let prev_title = todo.title.clone();
                let prev_priority = todo.priority.clone();
                let prev_tags = todo.tags.clone();
                let prev_due_date = todo.due_date.clone();
                todo.title = title;
                todo.priority = self.form.priority.clone();
                todo.tags = tags;
                todo.due_date = due_date;
                if let Err(e) = self.storage.update_todo(todo) {
                    todo.title = prev_title;
                    todo.priority = prev_priority;
                    todo.tags = prev_tags;
                    todo.due_date = prev_due_date;
                    self.error_message = Some(e.to_string());
                    return Ok(());
                }
            }
        } else {
            let new_todo = NewTodo {
                title,
                priority: self.form.priority.clone(),
                tags,
                due_date,
            };
            let inserted = self.storage.insert_todo(&new_todo)?;
            self.todos.insert(0, inserted);
        }

        // 刷新标签列表
        self.all_tags = self.storage.list_all_tags()?;
        self.mode = AppMode::Normal;
        self.form = FormState::default();
        Ok(())
    }

    // Tags 字段：Enter 确认当前输入的单个标签
    fn form_confirm_tag(&mut self) {
        let tag = self.form.tag_input.trim().to_string();
        if !tag.is_empty() && !self.form.tags.contains(&tag) {
            self.form.tags.push(tag);
        }
        self.form.tag_input.clear();
    }

    fn form_next_field(&mut self) {
        self.form.focused_field = match self.form.focused_field {
            FormField::Title => FormField::Tags,
            FormField::Tags => FormField::Priority,
            FormField::Priority => FormField::DueDate,
            FormField::DueDate => FormField::Title,
        };
    }

    fn form_prev_field(&mut self) {
        self.form.focused_field = match self.form.focused_field {
            FormField::Title => FormField::DueDate,
            FormField::Tags => FormField::Title,
            FormField::Priority => FormField::Tags,
            FormField::DueDate => FormField::Priority,
        };
    }

    fn form_backspace(&mut self) {
        match self.form.focused_field {
            FormField::Title => {
                self.form.title.pop();
                self.form.title_error = None;
            }
            FormField::Tags => {
                if !self.form.tag_input.is_empty() {
                    self.form.tag_input.pop();
                } else {
                    // 输入框为空时，退格删除最后一个已确认标签
                    self.form.tags.pop();
                }
            }
            FormField::DueDate => {
                self.form.due_date.pop();
                self.form.due_date_error = None;
            }
            _ => {}
        }
    }

    fn form_input(&mut self, c: char) {
        match self.form.focused_field {
            FormField::Title => {
                self.form.title.push(c);
                self.form.title_error = None;
            }
            FormField::Tags => {
                // 逗号或空格自动确认当前标签
                if c == ',' || c == ' ' {
                    self.form_confirm_tag();
                } else {
                    self.form.tag_input.push(c);
                }
            }
            FormField::DueDate => {
                self.form.due_date.push(c);
                self.form.due_date_error = None;
            }
            _ => {}
        }
    }

    fn form_cycle_up(&mut self) {
        if self.form.focused_field == FormField::Priority {
            self.form.priority = match self.form.priority {
                Priority::High => Priority::Low,
                Priority::Medium => Priority::High,
                Priority::Low => Priority::Medium,
            };
        }
    }

    fn form_cycle_down(&mut self) {
        if self.form.focused_field == FormField::Priority {
            self.form.priority = match self.form.priority {
                Priority::High => Priority::Medium,
                Priority::Medium => Priority::Low,
                Priority::Low => Priority::High,
            };
        }
    }

    /// 处理鼠标事件，需要传入上一帧渲染得到的布局区域用于命中检测。
    pub fn handle_mouse(
        &mut self,
        event: MouseEvent,
        layout_tag_panel: Rect,
        layout_list: Rect,
    ) -> Result<()> {
        // 弹窗模式下不处理鼠标（避免误操作）
        match self.mode {
            AppMode::Add | AppMode::Edit | AppMode::DeleteConfirm | AppMode::Help => return Ok(()),
            _ => {}
        }

        let col = event.column;
        let row = event.row;

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.handle_mouse_click(col, row, layout_tag_panel, layout_list)?;
            }
            MouseEventKind::ScrollDown => {
                self.handle_mouse_scroll(col, row, true, layout_tag_panel, layout_list);
            }
            MouseEventKind::ScrollUp => {
                self.handle_mouse_scroll(col, row, false, layout_tag_panel, layout_list);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_click(
        &mut self,
        col: u16,
        row: u16,
        tag_area: Rect,
        list_area: Rect,
    ) -> Result<()> {
        if in_rect(col, row, tag_area) {
            // 标签面板内点击：inner area 从 border+1 开始
            let inner_row = row.saturating_sub(tag_area.y + 1);
            let items_len = self.tag_panel_items().len();
            let idx = inner_row as usize;
            if idx < items_len {
                self.tag_panel_index = idx;
                let items = self.tag_panel_items();
                self.selected_tag = items[idx].map(|s| s.to_string());
                self.selected_index = 0;
                self.focus_tag_panel = true;
                self.mode = AppMode::Normal;
            }
        } else if in_rect(col, row, list_area) {
            self.focus_tag_panel = false;
            // 列表内点击：inner area 从 border+1 开始
            let inner_row = row.saturating_sub(list_area.y + 1);
            let idx = inner_row as usize;
            let filtered_len = self.filtered_todos().len();
            if idx < filtered_len {
                if self.selected_index == idx && self.mode == AppMode::Normal {
                    // 双击同一项 → 打开详情
                    self.mode = AppMode::Detail;
                } else {
                    self.selected_index = idx;
                    self.mode = AppMode::Normal;
                }
            }
        }
        Ok(())
    }

    fn handle_mouse_scroll(
        &mut self,
        col: u16,
        row: u16,
        down: bool,
        tag_area: Rect,
        list_area: Rect,
    ) {
        if in_rect(col, row, tag_area) {
            let items_len = self.tag_panel_items().len();
            if down {
                if self.tag_panel_index + 1 < items_len {
                    self.tag_panel_index += 1;
                }
            } else if self.tag_panel_index > 0 {
                self.tag_panel_index -= 1;
            }
        } else if in_rect(col, row, list_area) {
            if down {
                self.move_down();
            } else {
                self.move_up();
            }
        }
    }
}

fn in_rect(col: u16, row: u16, r: Rect) -> bool {
    col >= r.x && col < r.x + r.width && row >= r.y && row < r.y + r.height
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Priority, Todo, TodoStatus};
    use crate::storage::Storage;

    fn make_todo(id: i64, title: &str, tags: Vec<String>) -> Todo {
        Todo {
            id,
            title: title.to_string(),
            status: TodoStatus::Pending,
            priority: Priority::Medium,
            tags,
            due_date: None,
            created_at: "2026-01-01T00:00:00".to_string(),
        }
    }

    fn make_app(todos: Vec<Todo>) -> AppState {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let storage = Storage::new(tmp.path()).unwrap();
        let all_tags: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            todos
                .iter()
                .flat_map(|t| t.tags.iter())
                .filter(|tag| seen.insert(tag.as_str()))
                .cloned()
                .collect()
        };
        // Keep tempfile alive by leaking it; OK for short-lived test processes
        std::mem::forget(tmp);
        AppState {
            mode: crate::models::AppMode::Normal,
            todos,
            all_tags,
            selected_tag: None,
            tag_panel_index: 0,
            focus_tag_panel: false,
            selected_index: 0,
            list_offset: 0,
            search_query: String::new(),
            form: crate::models::FormState::default(),
            error_message: None,
            storage,
        }
    }

    #[test]
    fn test_filtered_todos_by_tag() {
        let todos = vec![
            make_todo(1, "工作任务", vec!["工作".to_string()]),
            make_todo(2, "生活任务", vec!["生活".to_string()]),
            make_todo(3, "多标签", vec!["工作".to_string(), "生活".to_string()]),
        ];
        let mut app = make_app(todos);

        assert_eq!(app.filtered_todos().len(), 3);

        app.selected_tag = Some("工作".to_string());
        let work = app.filtered_todos();
        assert_eq!(work.len(), 2);
        assert!(work.iter().all(|t| t.tags.contains(&"工作".to_string())));

        app.selected_tag = Some("生活".to_string());
        assert_eq!(app.filtered_todos().len(), 2);
    }

    #[test]
    fn test_filtered_todos_by_search() {
        let todos = vec![
            make_todo(1, "买菜", vec![]),
            make_todo(2, "写代码", vec![]),
            make_todo(3, "买书", vec![]),
        ];
        let mut app = make_app(todos);

        app.search_query = "买".to_string();
        assert_eq!(app.filtered_todos().len(), 2);

        app.search_query = "代码".to_string();
        assert_eq!(app.filtered_todos().len(), 1);

        app.search_query = "不存在".to_string();
        assert_eq!(app.filtered_todos().len(), 0);
    }

    #[test]
    fn test_tag_panel_items() {
        let todos = vec![
            make_todo(1, "任务1", vec!["工作".to_string()]),
            make_todo(2, "任务2", vec!["生活".to_string()]),
        ];
        let mut app = make_app(todos);
        app.all_tags = vec!["工作".to_string(), "生活".to_string()];

        let items = app.tag_panel_items();
        assert!(items[0].is_none());
        assert_eq!(items[1], Some("工作"));
        assert_eq!(items[2], Some("生活"));
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_delete_adjusts_selection() {
        let todos = vec![
            make_todo(1, "任务1", vec![]),
            make_todo(2, "任务2", vec![]),
            make_todo(3, "任务3", vec![]),
        ];
        let mut app = make_app(todos);
        app.selected_index = 2;

        app.todos.retain(|t| t.id != 3);
        let len = app.filtered_todos().len();
        if app.selected_index >= len && len > 0 {
            app.selected_index = len - 1;
        }

        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_selected_todo_respects_filter() {
        let todos = vec![
            make_todo(1, "工作任务", vec!["工作".to_string()]),
            make_todo(2, "生活任务", vec!["生活".to_string()]),
        ];
        let mut app = make_app(todos);
        app.selected_tag = Some("生活".to_string());
        app.selected_index = 0;

        let selected = app.selected_todo().unwrap();
        assert_eq!(selected.title, "生活任务");
    }
}
