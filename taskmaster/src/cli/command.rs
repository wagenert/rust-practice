use crate::tasks::task::TaskId;
use clap::Subcommand;

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
