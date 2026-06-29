use clap::Parser;
use taskmaster::cli::command::Command;
use taskmaster::cli::parser::Cli;
use taskmaster::tasks::task::Task;
use taskmaster::tasks::task::TaskId;
use taskmaster::tasks::task_storage::TaskStorage;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let task_storage = TaskStorage::new(&cli.filename);
    match cli.command {
        Command::List => list_tasks(&task_storage),
        Command::Create { title } => create_task(&task_storage, title),
        Command::MarkDone { task_id } => mark_task_done(&task_storage, task_id),
    }
}

fn mark_task_done(task_storage: &TaskStorage, task_id: u32) -> Result<(), std::io::Error> {
    let mut tasks = task_storage.read()?;

    if let Some(task) = tasks.get_mut(task_id as usize) {
        task.done();
    } else {
        println!("Task with id {task_id} not found.");
    }
    task_storage.write(&tasks)
}

fn create_task(task_storage: &TaskStorage, title: String) -> Result<(), std::io::Error> {
    let mut tasks = task_storage.read()?;

    let task_id = tasks.len();
    let new_task = Task::new(task_id as TaskId, title);
    tasks.add_task(new_task);
    println!("tasks: {:?}", tasks);
    task_storage.write(&tasks)
}

fn list_tasks(task_storage: &TaskStorage) -> Result<(), std::io::Error> {
    match task_storage.read() {
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
