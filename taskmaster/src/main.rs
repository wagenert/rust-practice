mod task;

use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use clap::{Parser, Subcommand};
use task::TaskId;

use crate::task::Task;

#[derive(Parser)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
    /// Json file to use
    #[arg(long, short)]
    filename: String,
}

#[derive(Clone, Subcommand)]
enum Command {
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

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::List => {
            let filepath = std::path::Path::new(&cli.filename);
            if filepath.exists() {
                let file = OpenOptions::new()
                    .read(true)
                    .create(false)
                    .open(cli.filename)?;
                let read_buf = BufReader::new(&file);
                let tasks: Vec<Task> = serde_json::from_reader(read_buf)?;
                println!("Tasks");
                for task in tasks.iter() {
                    println!("{task}");
                }
            } else {
                println!("File does not exist. No tasks found.");
            }
        }
        Command::Create { title } => {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(cli.filename)?;
            let read_buf = BufReader::new(&file);
            let mut tasks: Vec<Task> = match serde_json::from_reader(read_buf) {
                Ok(tasks) => tasks,
                Err(err) => {
                    if err.is_eof() {
                        Vec::new()
                    } else {
                        panic!("Can not process file!");
                    }
                }
            };

            let task_id = tasks.len();
            let new_task = Task::new(task_id as TaskId, title);
            tasks.push(new_task);
            let write_buf = BufWriter::new(file);
            serde_json::to_writer(write_buf, &tasks)?;
        }
        Command::MarkDone { task_id } => {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(false)
                .truncate(true)
                .open(cli.filename)?;
            let read_buf = BufReader::new(&file);
            let mut tasks: Vec<Task> = match serde_json::from_reader(read_buf) {
                Ok(tasks) => tasks,
                Err(err) => {
                    if err.is_eof() {
                        Vec::new()
                    } else {
                        panic!("Can not process file!");
                    }
                }
            };

            if let Some(task) = tasks.get_mut(task_id as usize) {
                task.done();
            } else {
                println!("Task with id {task_id} not found.");
            }
            let write_buf = BufWriter::new(file);
            serde_json::to_writer(write_buf, &tasks)?;
        }
    }
    Ok(())
}
