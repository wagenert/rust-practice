use std::collections::HashMap;

use crate::tasks::task::{Task, TaskId};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug)]
pub struct TaskList {
    tasks: HashMap<TaskId, Task>,
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id(), task);
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, TaskId, Task> {
        self.tasks.iter()
    }

    pub fn get_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_list_add_and_get_by_id() {
        let mut task_list = TaskList::default();
        assert!(task_list.is_empty());

        let task_id = uuid::Uuid::new_v4();
        let task = Task::new(task_id, "Test Task".to_string());
        task_list.add_task(task);

        assert_eq!(task_list.len(), 1);

        let stored_task = task_list
            .get_mut(task_id)
            .expect("task with id should exist");
        assert_eq!(stored_task.title(), "Test Task");
        assert!(!stored_task.is_done());
    }

    #[test]
    fn test_task_list_marks_task_done_by_id() {
        let mut task_list = TaskList::default();
        let task_id = uuid::Uuid::new_v4();
        task_list.add_task(Task::new(task_id, "HashMap lookup".to_string()));

        let task = task_list
            .get_mut(task_id)
            .expect("task with id 42 should be retrievable");
        task.done();

        let task = task_list
            .get_mut(task_id)
            .expect("task with id 42 should still exist");
        assert!(task.is_done());
    }

    #[test]
    fn test_task_list_replaces_existing_task_with_same_id() {
        let mut task_list = TaskList::default();

        let task_id = uuid::Uuid::new_v4();
        task_list.add_task(Task::new(task_id, "First".to_string()));
        task_list.add_task(Task::new(task_id, "First".to_string()));
        task_list.add_task(Task::new(task_id, "Second".to_string()));

        assert_eq!(task_list.len(), 1);
        let task = task_list
            .get_mut(task_id)
            .expect("task with id should exist after replacement");
        assert_eq!(task.title(), "Second");
    }
}
