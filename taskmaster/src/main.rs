use anyhow::Result;
use clap::Parser;
use taskmaster::cli::command::{Command, create_task, list_tasks, mark_task_done};
use taskmaster::cli::parser::Cli;
use taskmaster::tasks::task_storage::TaskStorage;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}

fn run(cli: Cli) -> Result<()> {
    let task_storage = TaskStorage::new(&cli.filename);
    match cli.command {
        Command::List => list_tasks(&task_storage),
        Command::Create { title } => create_task(&task_storage, title),
        Command::MarkDone { task_id } => mark_task_done(&task_storage, task_id),
    }
}
