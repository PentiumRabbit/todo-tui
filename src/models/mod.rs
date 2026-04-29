pub mod todo;

use chrono;

pub use todo::{NewTodo, Priority, Todo, TodoStatus};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Detail,
    Add,
    Edit,
    DeleteConfirm,
    Search,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormField {
    Title,
    Tags,
    Priority,
    DueDate,
}

#[derive(Debug, Clone)]
pub struct FormState {
    pub title: String,
    pub tags: Vec<String>,       // 已选标签列表
    pub tag_input: String,       // 当前正在输入的标签
    pub priority: Priority,
    pub due_date: String,
    pub focused_field: FormField,
    pub editing_todo_id: Option<i64>,
    pub title_error: Option<String>,
    pub due_date_error: Option<String>,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            title: String::new(),
            tags: Vec::new(),
            tag_input: String::new(),
            priority: Priority::Medium,
            due_date: (chrono::Local::now() + chrono::Duration::days(1))
                .format("%Y-%m-%d")
                .to_string(),
            focused_field: FormField::Title,
            editing_todo_id: None,
            title_error: None,
            due_date_error: None,
        }
    }
}

impl FormState {
    pub fn from_todo(todo: &Todo) -> Self {
        Self {
            title: todo.title.clone(),
            tags: todo.tags.clone(),
            tag_input: String::new(),
            priority: todo.priority.clone(),
            due_date: todo.due_date.clone().unwrap_or_else(|| {
                (chrono::Local::now() + chrono::Duration::days(1))
                    .format("%Y-%m-%d")
                    .to_string()
            }),
            focused_field: FormField::Title,
            editing_todo_id: Some(todo.id),
            title_error: None,
            due_date_error: None,
        }
    }
}

pub enum AppAction {
    Continue,
    Quit,
}
