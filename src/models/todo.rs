use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        }
    }

    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            Priority::High => "高",
            Priority::Medium => "中",
            Priority::Low => "低",
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "High" => Ok(Priority::High),
            "Medium" => Ok(Priority::Medium),
            "Low" => Ok(Priority::Low),
            _ => Ok(Priority::Medium),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    Pending,
    Done,
    Cancelled,
}

impl TodoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoStatus::Pending => "Pending",
            TodoStatus::Done => "Done",
            TodoStatus::Cancelled => "Cancelled",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            TodoStatus::Pending => TodoStatus::Done,
            TodoStatus::Done => TodoStatus::Pending,
            TodoStatus::Cancelled => TodoStatus::Pending,
        }
    }
}

impl std::str::FromStr for TodoStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Done" => Ok(TodoStatus::Done),
            "Cancelled" => Ok(TodoStatus::Cancelled),
            _ => Ok(TodoStatus::Pending),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: TodoStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl Todo {
    #[allow(dead_code)]
    pub fn is_completed(&self) -> bool {
        self.status == TodoStatus::Done
    }

    #[allow(dead_code)]
    pub fn is_cancelled(&self) -> bool {
        self.status == TodoStatus::Cancelled
    }

    fn parse_due(&self) -> Option<NaiveDateTime> {
        let s = self.due_date.as_deref()?;
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
            return Some(dt);
        }
        let d = NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()?;
        d.and_hms_opt(0, 0, 0)
    }

    pub fn is_overdue(&self) -> bool {
        if self.status != TodoStatus::Pending {
            return false;
        }
        self.parse_due()
            .is_some_and(|due| due < chrono::Local::now().naive_local())
    }

    pub fn is_due_today(&self) -> bool {
        if self.status != TodoStatus::Pending {
            return false;
        }
        self.parse_due()
            .is_some_and(|due| due.date() == chrono::Local::now().date_naive())
    }
}

pub struct NewTodo {
    pub title: String,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    fn make_todo(status: TodoStatus, due_date: Option<&str>) -> Todo {
        Todo {
            id: 1,
            title: "测试任务".to_string(),
            status,
            priority: Priority::Medium,
            tags: vec![],
            due_date: due_date.map(|s| s.to_string()),
            notes: None,
            created_at: "2026-01-01T00:00:00".to_string(),
        }
    }

    #[test]
    fn test_todo_is_overdue_boundary() {
        let now = Local::now().naive_local();
        let today = now.date();

        // 无截止日期 → 不过期
        let todo = make_todo(TodoStatus::Pending, None);
        assert!(!todo.is_overdue());

        // 已完成任务 → 不过期（即使截止日期在过去）
        let past = (today - Duration::days(1)).format("%Y-%m-%d").to_string();
        let todo = make_todo(TodoStatus::Done, Some(&past));
        assert!(!todo.is_overdue());

        // 未来日期 → 不过期
        let future = (today + Duration::days(1)).format("%Y-%m-%d").to_string();
        let todo = make_todo(TodoStatus::Pending, Some(&future));
        assert!(!todo.is_overdue());

        // 过去日期 → 过期
        let todo = make_todo(TodoStatus::Pending, Some(&past));
        assert!(todo.is_overdue());
    }

    #[test]
    fn test_todo_is_due_today_boundary() {
        let now = Local::now().naive_local();
        let today = now.date();

        // 今天 → 是今天到期
        let today_str = today.format("%Y-%m-%d").to_string();
        let todo = make_todo(TodoStatus::Pending, Some(&today_str));
        assert!(todo.is_due_today());

        // 非今天（明天）→ 不是今天到期
        let tomorrow = (today + Duration::days(1)).format("%Y-%m-%d").to_string();
        let todo = make_todo(TodoStatus::Pending, Some(&tomorrow));
        assert!(!todo.is_due_today());

        // 已完成任务 → 不是今天到期（即使截止日期是今天）
        let todo = make_todo(TodoStatus::Done, Some(&today_str));
        assert!(!todo.is_due_today());

        // 无截止日期 → 不是今天到期
        let todo = make_todo(TodoStatus::Pending, None);
        assert!(!todo.is_due_today());
    }

    #[test]
    fn test_todo_defaults_and_status() {
        // 正常路径：构造 Todo 并验证字段默认值、优先级/状态转换、is_completed 判断
        let todo = make_todo(TodoStatus::Pending, None);
        assert_eq!(todo.title, "测试任务");
        assert_eq!(todo.priority.as_str(), "中");
        assert_eq!(todo.status.as_str(), "Pending");
        assert!(!todo.is_completed());

        // 验证 label() 方法
        assert_eq!(todo.priority.label(), "Medium");
    }

    #[test]
    fn test_todo_empty_title() {
        // 边界/异常路径：空标题构造 Todo
        let todo = Todo {
            id: 1,
            title: "".to_string(),
            status: TodoStatus::Pending,
            priority: Priority::Medium,
            tags: vec![],
            due_date: None,
            notes: None,
            created_at: "2026-01-01T00:00:00".to_string(),
        };
        assert_eq!(todo.title, "");
        assert_eq!(todo.status.as_str(), "Pending");
        assert!(!todo.is_completed());
    }

}

