mod task;
mod task_list;

use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use clap::{Parser, Subcommand};
use task::TaskId;

use crate::task::Task;
use crate::task_list::TaskList;

#[derive(Parser)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
    /// Json file to use
    #[arg(long, short)]
    filename: String,
}

#[derive(Clone, Subcommand, Debug)]
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
                let tasks: TaskList = serde_json::from_reader(read_buf)?;
                println!("Tasks");
                for task in tasks.iter() {
                    println!("{task}");
                }
            } else {
                println!("File does not exist. No tasks found.");
            }
            Ok(())
        }
        Command::Create { title } => {
            let mut tasks = read_tasklist(&cli.filename)?;

            let task_id = tasks.len();
            let new_task = Task::new(task_id as TaskId, title);
            tasks.push(new_task);
            println!("tasks: {:?}", tasks);
            write_takslist(&cli.filename, &tasks)
        }
        Command::MarkDone { task_id } => {
            let mut tasks = read_tasklist(&cli.filename)?;

            if let Some(task) = tasks.get_mut(task_id as usize) {
                task.done();
            } else {
                println!("Task with id {task_id} not found.");
            }
            write_takslist(&cli.filename, &tasks)
        }
    }
}

fn read_tasklist(filename: &str) -> Result<TaskList, std::io::Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(filename)?;
    let read_buf = BufReader::new(&file);
    let tasks: TaskList = match serde_json::from_reader(read_buf) {
        Ok(tasks) => tasks,
        Err(err) => {
            if err.is_eof() {
                TaskList::default()
            } else {
                panic!("Can not process file!");
            }
        }
    };
    Ok(tasks)
}

fn write_takslist(filename: &str, tasks: &TaskList) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open(filename)?;
    let write_buf = BufWriter::new(&file);
    serde_json::to_writer(write_buf, &tasks)?;
    Ok(())
}
