mod task;

use clap::{Parser, Subcommand};
use task::TaskId;

use crate::task::Task;

#[derive(Parser)]
struct Cli {
    /// Command to execute
    #[command(subcommand)]
    command: Command,
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

static mut CURRENT_ID: TaskId = 0;
static mut TASKS: Vec<Task> = vec![];

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::List => {
            println!("Tasks");
            for task in unsafe { TASKS.clone() } {
                println!("{task}");
            }
        }
        Command::Create { title } => {
            let task_id = unsafe { CURRENT_ID };
            unsafe { CURRENT_ID += 1 };
            let new_task = Task::new(task_id, title);
            unsafe { TASKS.push(new_task) };
        }
        Command::MarkDone { task_id } => {
            if let Some(task) = unsafe { TASKS.get_mut(task_id as usize) } {
                task.done();
            }
        }
    }
}
