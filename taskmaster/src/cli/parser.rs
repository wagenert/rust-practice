use crate::cli::command::Command;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
    /// Json file to use
    #[arg(long, short)]
    pub filename: String,
}
