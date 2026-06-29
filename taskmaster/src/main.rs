use clap::{Parser, Subcommand};
use taskmaster::task::Task;
use taskmaster::task::TaskId;
use taskmaster::task_list::TaskList;

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
        Command::List => match TaskList::read(&cli.filename) {
            Ok(tasks) => {
                println!("Tasks");
                for task in tasks.iter() {
                    println!("{task}");
                }
                Ok(())
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("File does not exist. No tasks found.");
                    Ok(())
                }
                _ => {
                    println!("Error reading file: {}", err);
                    Err(err)
                }
            },
        },
        Command::Create { title } => {
            let mut tasks = TaskList::read(&cli.filename)?;

            let task_id = tasks.len();
            let new_task = Task::new(task_id as TaskId, title);
            tasks.add_task(new_task);
            println!("tasks: {:?}", tasks);
            tasks.write(&cli.filename)
        }
        Command::MarkDone { task_id } => {
            let mut tasks = TaskList::read(&cli.filename)?;

            if let Some(task) = tasks.get_mut(task_id as usize) {
                task.done();
            } else {
                println!("Task with id {task_id} not found.");
            }
            tasks.write(&cli.filename)
        }
    }
}
