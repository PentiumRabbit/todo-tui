pub mod todo;

use chrono::Datelike;
use chrono::Timelike;
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
    Notes,
    Tags,
    Priority,
    DueDate,
}

/// 截止时间分段，对应 年/月/日/时/分
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DueDateSegment {
    Year,
    Month,
    Day,
    Hour,
    Minute,
}

impl DueDateSegment {
    pub fn prev(self) -> Self {
        match self {
            Self::Year => Self::Year,
            Self::Month => Self::Year,
            Self::Day => Self::Month,
            Self::Hour => Self::Day,
            Self::Minute => Self::Hour,
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::Year => Self::Month,
            Self::Month => Self::Day,
            Self::Day => Self::Hour,
            Self::Hour => Self::Minute,
            Self::Minute => Self::Minute,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormState {
    pub title: String,
    pub notes: String,
    pub tags: Vec<String>,         // 已选标签列表
    pub tag_input: String,         // 当前正在输入的标签
    pub tag_cursor: Option<usize>, // 光标指向的 tag 索引（None = 输入态）
    pub priority: Priority,
    pub due_year: i32,
    pub due_month: u32,
    pub due_day: u32,
    pub due_hour: u32,
    pub due_minute: u32,
    pub due_enabled: bool,
    pub due_segment: DueDateSegment,
    pub focused_field: FormField,
    pub editing_todo_id: Option<i64>,
    pub title_error: Option<String>,
}

impl FormState {
    fn default_due() -> chrono::NaiveDateTime {
        (chrono::Local::now() + chrono::Duration::days(1)).naive_local()
    }

    /// 从存储字符串解析截止时间，兼容 `YYYY-MM-DD` 和 `YYYY-MM-DD HH:MM`。
    pub fn parse_due_date(s: &str) -> Option<chrono::NaiveDateTime> {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
            return Some(dt);
        }
        if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return d.and_hms_opt(0, 0, 0);
        }
        None
    }

    /// 将分段值格式化为存储字符串 `YYYY-MM-DD HH:MM`。
    pub fn due_to_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.due_year, self.due_month, self.due_day, self.due_hour, self.due_minute
        )
    }
}

impl Default for FormState {
    fn default() -> Self {
        let dt = Self::default_due();
        Self {
            title: String::new(),
            notes: String::new(),
            tags: Vec::new(),
            tag_input: String::new(),
            tag_cursor: None,
            priority: Priority::Low,
            due_year: dt.date().year(),
            due_month: dt.date().month(),
            due_day: dt.date().day(),
            due_hour: dt.time().hour(),
            due_minute: dt.time().minute(),
            due_enabled: true,
            due_segment: DueDateSegment::Day,
            focused_field: FormField::Title,
            editing_todo_id: None,
            title_error: None,
        }
    }
}

impl FormState {
    pub fn from_todo(todo: &Todo) -> Self {
        let dt = todo
            .due_date
            .as_deref()
            .and_then(Self::parse_due_date)
            .unwrap_or_else(Self::default_due);
        let due_enabled = todo.due_date.is_some();
        Self {
            title: todo.title.clone(),
            notes: todo.notes.clone().unwrap_or_default(),
            tags: todo.tags.clone(),
            tag_input: String::new(),
            tag_cursor: None,
            priority: todo.priority.clone(),
            due_year: dt.date().year(),
            due_month: dt.date().month(),
            due_day: dt.date().day(),
            due_hour: dt.time().hour(),
            due_minute: dt.time().minute(),
            due_enabled,
            due_segment: DueDateSegment::Day,
            focused_field: FormField::Title,
            editing_todo_id: Some(todo.id),
            title_error: None,
        }
    }
}

pub enum AppAction {
    Continue,
    Quit,
}
