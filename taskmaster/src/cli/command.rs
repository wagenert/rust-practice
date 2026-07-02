use crate::tasks::{
    task::{Task, TaskId},
    task_storage::TaskStorage,
};
use anyhow::Result;
use clap::Subcommand;

#[derive(Clone, Subcommand, Debug)]
pub enum Command {
    /// List all tasks
    List,
    /// Create new tasks
    Create {
        /// Title of the new task
        title: String,
    },
    /// Mark a task as done
    MarkDone {
        /// task_id to mark as done
        task_id: TaskId,
    },
}

pub fn mark_task_done(task_storage: &dyn TaskStorage, task_id: TaskId) -> Result<()> {
    let mut tasks = task_storage.load()?;

    if let Some(task) = tasks.get_mut(task_id) {
        task.mark_done();
        task_storage.save(&tasks)
    } else {
        println!("Task with id {task_id} not found.");
        Ok(())
    }
}

pub fn create_task(task_storage: &dyn TaskStorage, title: String) -> Result<()> {
    let mut tasks = task_storage.load()?;

    let task_id = uuid::Uuid::new_v4();
    let new_task = Task::new(task_id, title);
    tasks.add_task(new_task);
    task_storage.save(&tasks)
}

pub fn list_tasks(task_storage: &dyn TaskStorage) -> Result<()> {
    let tasks = task_storage.load()?;
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks.iter() {
            let status = if task.is_done() { "✓" } else { " " };
            println!("[{}]: {}", status, task.title());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task_storage::JsonFileTaskStorage;
    use std::{
        fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_storage() -> (JsonFileTaskStorage, String) {
        let id = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let filename = format!("test_tasks_command_{id}.json");
        (JsonFileTaskStorage::new(&filename), filename)
    }

    fn cleanup(filename: &str) {
        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_create_task() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        let task_storage = &task_storage as &dyn TaskStorage;
        let result = create_task(task_storage, title.clone());
        assert!(result.is_ok());

        let tasks = task_storage.load().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.iter().next().unwrap().title(), &title);
        cleanup(&filename);
    }

    #[test]
    fn test_mark_task_done() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        let task_storage = &task_storage as &dyn TaskStorage;
        create_task(task_storage, title.clone()).unwrap();

        let task_id = task_storage.load().unwrap().iter().next().unwrap().id();
        let result = mark_task_done(task_storage, task_id);
        assert!(result.is_ok());

        let tasks = task_storage.load().unwrap();
        assert!(tasks.iter().next().unwrap().is_done());
        cleanup(&filename);
    }

    #[test]
    fn test_list_tasks() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        let task_storage = &task_storage as &dyn TaskStorage;
        create_task(task_storage, title.clone()).unwrap();

        let result = list_tasks(task_storage);
        assert!(result.is_ok());
        cleanup(&filename);
    }
}
