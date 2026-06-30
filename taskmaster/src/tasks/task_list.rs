use crate::tasks::task::Task;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Task> {
        self.tasks.iter()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_list() {
        let mut task_list = TaskList::default();
        assert!(task_list.is_empty());

        let task = Task::new(0, "Test Task".to_string());
        task_list.add_task(task);
        assert_eq!(task_list.len(), 1);
    }
}
