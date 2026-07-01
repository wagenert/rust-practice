use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    title: String,
    done: bool,
}

impl Task {
    pub fn new(id: TaskId, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }

    pub fn done(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> TaskId {
        self.id
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {{ id: {}, title: {}, done: {} }}",
            self.id(),
            self.title(),
            self.is_done()
        )
    }
}

pub type TaskId = uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(uuid::Uuid::new_v4(), "Test Task".to_string());
        assert_eq!(task.title(), "Test Task");
        assert!(!task.is_done());
    }

    #[test]
    fn test_task_done() {
        let mut task = Task::new(uuid::Uuid::new_v4(), "Test Task".to_string());
        task.done();
        assert!(task.is_done());
    }
}
