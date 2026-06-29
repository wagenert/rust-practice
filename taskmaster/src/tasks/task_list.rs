use std::ops::{Deref, DerefMut};

use crate::tasks::task::Task;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug)]
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
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut task_list = TaskList::default();
        let task = Task::new(1, "Test Task".to_string());
        task_list.add_task(task.clone());
        assert_eq!(task_list.len(), 1);
        assert_eq!(task_list[0], task);
    }

    #[test]
    fn test_read_write() {
        let filename = "test_tasks.json";
        let mut task_list = TaskList::default();
        let task = Task::new(1, "Test Task".to_string());
        task_list.add_task(task);
        task_list.write(filename).unwrap();
        let read_task_list = TaskList::read(filename).unwrap();
        assert_eq!(task_list.len(), read_task_list.len());
        assert_eq!(task_list, read_task_list);
    }
}
