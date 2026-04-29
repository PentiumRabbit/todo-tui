use chrono::NaiveDate;
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
    pub created_at: String,
}

impl Todo {
    pub fn is_completed(&self) -> bool {
        self.status == TodoStatus::Done
    }

    pub fn is_cancelled(&self) -> bool {
        self.status == TodoStatus::Cancelled
    }

    pub fn is_overdue(&self) -> bool {
        if self.status != TodoStatus::Pending {
            return false;
        }
        self.due_date.as_deref().and_then(|d| d.parse::<NaiveDate>().ok()).map_or(false, |due| {
            due < chrono::Local::now().date_naive()
        })
    }

    pub fn is_due_today(&self) -> bool {
        if self.status != TodoStatus::Pending {
            return false;
        }
        self.due_date.as_deref().and_then(|d| d.parse::<NaiveDate>().ok()).map_or(false, |due| {
            due == chrono::Local::now().date_naive()
        })
    }
}

pub struct NewTodo {
    pub title: String,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
}
