use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Error},
};

use crate::tasks::task_list::TaskList;

pub struct TaskStorage<'a> {
    filename: &'a str,
}

impl<'a> TaskStorage<'a> {
    pub fn new(filename: &'a str) -> Self {
        Self { filename }
    }

    pub fn read(&self) -> Result<TaskList, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.filename)?;
        let read_buf = BufReader::new(&file);
        let tasks = match serde_json::from_reader(read_buf) {
            Ok(tasks) => Ok(tasks),
            Err(err) => {
                if err.is_eof() {
                    Ok(TaskList::default())
                } else {
                    Err(Error::new(std::io::ErrorKind::InvalidData, err))
                }
            }
        }?;
        Ok(tasks)
    }

    pub fn write(&self, task_list: &TaskList) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.filename)?;
        let write_buf = BufWriter::new(&file);
        serde_json::to_writer(write_buf, task_list)?;
        Ok(())
    }
}
