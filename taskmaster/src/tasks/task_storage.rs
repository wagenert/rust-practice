use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use anyhow::Result;

use crate::tasks::task_list::TaskList;

pub trait TaskStorage {
    fn read(&self) -> Result<TaskList>;
    fn write(&self, task_list: &TaskList) -> Result<()>;
}
pub struct JsonFileTaskStorage<'a> {
    filename: &'a str,
}

impl<'a> JsonFileTaskStorage<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self { filename }
    }
}

impl<'a> TaskStorage for JsonFileTaskStorage<'a> {
    fn read(&self) -> Result<TaskList> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.filename)?;
        file.lock_shared()?;
        let read_buf = BufReader::new(&file);
        let tasks = match serde_json::from_reader(read_buf) {
            Ok(tasks) => Ok(tasks),
            Err(err) => {
                if err.is_eof() {
                    Ok(TaskList::default())
                } else {
                    Err(anyhow::anyhow!(err))
                }
            }
        }?;
        Ok(tasks)
    }

    fn write(&self, task_list: &TaskList) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.filename)?;
        file.lock()?;
        let write_buf = BufWriter::new(&file);
        serde_json::to_writer(write_buf, task_list)?;
        Ok(())
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

    fn test_filename() -> String {
        let id = TEST_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("test_tasks_storage_{id}.json")
    }

    #[test]
    fn test_read_write() {
        let filename = test_filename();
        let storage = JsonFileTaskStorage::new(&filename);
        let mut task_list = TaskList::default();
        let task = crate::tasks::task::Task::new(uuid::Uuid::new_v4(), "Test Task".to_string());
        task_list.add_task(task);
        storage.write(&task_list).unwrap();
        let read_task_list = storage.read().unwrap();
        assert_eq!(task_list.len(), read_task_list.len());
        assert_eq!(task_list, read_task_list);
        let _ = fs::remove_file(&filename);
    }
}
