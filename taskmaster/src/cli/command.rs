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

pub fn mark_task_done(task_storage: &TaskStorage, task_id: TaskId) -> Result<()> {
    let mut tasks = task_storage.read()?;

    if let Some(task) = tasks.get_mut(task_id) {
        task.done();
    } else {
        println!("Task with id {task_id} not found.");
    }
    task_storage.write(&tasks)
}

pub fn create_task(task_storage: &TaskStorage, title: String) -> Result<()> {
    let mut tasks = task_storage.read()?;

    let task_id = uuid::Uuid::new_v4();
    let new_task = Task::new(task_id, title);
    tasks.add_task(new_task);
    println!("tasks: {:?}", tasks);
    task_storage.write(&tasks)
}

pub fn list_tasks(task_storage: &TaskStorage) -> Result<()> {
    match task_storage.read() {
        Ok(tasks) => {
            println!("Tasks");
            for (_, task) in tasks.iter() {
                println!("{task}");
            }
            Ok(())
        }
        Err(err) => match err.downcast_ref::<std::io::Error>() {
            Some(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                println!("File does not exist. No tasks found.");
                Ok(())
            }
            _ => {
                println!("Error reading file: {}", err);
                Err(anyhow::anyhow!(err))
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEST_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_storage() -> (TaskStorage<'static>, &'static str) {
        let id = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let filename = format!("test_tasks_command_{id}.json");
        let filename = Box::leak(filename.into_boxed_str());
        (TaskStorage::new(filename), filename)
    }

    fn cleanup(filename: &str) {
        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_create_task() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        let result = create_task(&task_storage, title.clone());
        assert!(result.is_ok());

        let tasks = task_storage.read().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks.iter().next().unwrap().1.title(), &title);
        cleanup(filename);
    }

    #[test]
    fn test_mark_task_done() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        create_task(&task_storage, title.clone()).unwrap();

        let task_id = task_storage.read().unwrap().iter().next().unwrap().1.id();
        let result = mark_task_done(&task_storage, task_id);
        assert!(result.is_ok());

        let tasks = task_storage.read().unwrap();
        assert!(tasks.iter().next().unwrap().1.is_done());
        cleanup(filename);
    }

    #[test]
    fn test_list_tasks() {
        let (task_storage, filename) = test_storage();
        let title = "Test Task".to_string();
        create_task(&task_storage, title.clone()).unwrap();

        let result = list_tasks(&task_storage);
        assert!(result.is_ok());
        cleanup(filename);
    }
}
