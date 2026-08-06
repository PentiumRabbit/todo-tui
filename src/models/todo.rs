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

    #[test]
    fn priority_as_str() {
        assert_eq!(Priority::High.as_str(), "High");
        assert_eq!(Priority::Medium.as_str(), "Medium");
        assert_eq!(Priority::Low.as_str(), "Low");
    }

    #[test]
    fn priority_label() {
        assert_eq!(Priority::High.label(), "高");
        assert_eq!(Priority::Medium.label(), "中");
        assert_eq!(Priority::Low.label(), "低");
    }

    #[test]
    fn priority_from_str() {
        assert_eq!("High".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("Medium".parse::<Priority>().unwrap(), Priority::Medium);
        assert_eq!("Low".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("Unknown".parse::<Priority>().unwrap(), Priority::Medium);
    }

    #[test]
    fn todo_status_as_str() {
        assert_eq!(TodoStatus::Pending.as_str(), "Pending");
        assert_eq!(TodoStatus::Done.as_str(), "Done");
        assert_eq!(TodoStatus::Cancelled.as_str(), "Cancelled");
    }

    #[test]
    fn todo_status_next() {
        assert_eq!(TodoStatus::Pending.next(), TodoStatus::Done);
        assert_eq!(TodoStatus::Done.next(), TodoStatus::Pending);
        assert_eq!(TodoStatus::Cancelled.next(), TodoStatus::Pending);
    }

    #[test]
    fn todo_status_from_str() {
        assert_eq!("Done".parse::<TodoStatus>().unwrap(), TodoStatus::Done);
        assert_eq!("Cancelled".parse::<TodoStatus>().unwrap(), TodoStatus::Cancelled);
        assert_eq!("Unknown".parse::<TodoStatus>().unwrap(), TodoStatus::Pending);
    }

    fn make_todo(status: TodoStatus, due_date: Option<&str>) -> Todo {
        Todo {
            id: 1,
            title: "Test".to_string(),
            status,
            priority: Priority::Medium,
            tags: vec!["work".to_string()],
            due_date: due_date.map(|s| s.to_string()),
            notes: None,
            created_at: "2026-01-01 00:00:00".to_string(),
        }
    }

    #[test]
    fn todo_is_completed() {
        assert!(make_todo(TodoStatus::Done, None).is_completed());
        assert!(!make_todo(TodoStatus::Pending, None).is_completed());
        assert!(!make_todo(TodoStatus::Cancelled, None).is_completed());
    }

    #[test]
    fn todo_is_cancelled() {
        assert!(make_todo(TodoStatus::Cancelled, None).is_cancelled());
        assert!(!make_todo(TodoStatus::Pending, None).is_cancelled());
        assert!(!make_todo(TodoStatus::Done, None).is_cancelled());
    }

    #[test]
    fn todo_is_overdue_pending_with_past_date() {
        let todo = make_todo(TodoStatus::Pending, Some("2000-01-01"));
        assert!(todo.is_overdue());
    }

    #[test]
    fn todo_is_overdue_pending_with_future_date() {
        let todo = make_todo(TodoStatus::Pending, Some("2999-01-01"));
        assert!(!todo.is_overdue());
    }

    #[test]
    fn todo_is_overdue_non_pending() {
        assert!(!make_todo(TodoStatus::Done, Some("2000-01-01")).is_overdue());
        assert!(!make_todo(TodoStatus::Cancelled, Some("2000-01-01")).is_overdue());
    }

    #[test]
    fn todo_is_overdue_no_due_date() {
        assert!(!make_todo(TodoStatus::Pending, None).is_overdue());
    }

    #[test]
    fn todo_is_due_today_pending() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let todo = make_todo(TodoStatus::Pending, Some(&today));
        assert!(todo.is_due_today());
    }

    #[test]
    fn todo_is_due_today_non_pending() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        assert!(!make_todo(TodoStatus::Done, Some(&today)).is_due_today());
        assert!(!make_todo(TodoStatus::Cancelled, Some(&today)).is_due_today());
    }

    #[test]
    fn todo_is_due_today_no_due_date() {
        assert!(!make_todo(TodoStatus::Pending, None).is_due_today());
    }

    #[test]
    fn todo_is_due_today_datetime_format() {
        let today = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        let todo = make_todo(TodoStatus::Pending, Some(&today));
        assert!(todo.is_due_today());
    }
}
