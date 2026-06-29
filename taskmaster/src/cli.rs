use crate::tasks::task::TaskId;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
    /// Json file to use
    #[arg(long, short)]
    pub filename: String,
}

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
