use std::ops::{Deref, DerefMut};

use crate::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl Deref for TaskList {
    type Target = [Task];

    fn deref(&self) -> &Self::Target {
        &self.tasks
    }
}

impl DerefMut for TaskList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tasks
    }
}

impl TaskList {
    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
