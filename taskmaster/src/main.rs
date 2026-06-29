use clap::Parser;
use taskmaster::cli::command::Command;
use taskmaster::cli::parser::Cli;
use taskmaster::tasks::task::Task;
use taskmaster::tasks::task::TaskId;
use taskmaster::tasks::task_list::TaskList;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::List => list_tasks(&cli.filename),
        Command::Create { title } => create_task(&cli.filename, title),
        Command::MarkDone { task_id } => mark_task_done(&cli.filename, task_id),
    }
}

fn mark_task_done(filename: &str, task_id: u32) -> Result<(), std::io::Error> {
    let mut tasks = TaskList::read(filename)?;

    if let Some(task) = tasks.get_mut(task_id as usize) {
        task.done();
    } else {
        println!("Task with id {task_id} not found.");
    }
    tasks.write(filename)
}

fn create_task(filename: &str, title: String) -> Result<(), std::io::Error> {
    let mut tasks = TaskList::read(filename)?;

    let task_id = tasks.len();
    let new_task = Task::new(task_id as TaskId, title);
    tasks.add_task(new_task);
    println!("tasks: {:?}", tasks);
    tasks.write(filename)
}

fn list_tasks(filename: &str) -> Result<(), std::io::Error> {
    match TaskList::read(filename) {
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
    }
}
