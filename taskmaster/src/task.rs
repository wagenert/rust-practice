use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {{ id: {}, title: {}, done: {} }}",
            self.id, self.title, self.done
        )
    }
}

pub type TaskId = u32;
