use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new todo
    Create(CreateArg),

    /// List all todos
    List,
}

#[derive(Debug, Args, Clone)]
pub struct CreateArg {
    /// Name of the todo item
    pub item: String,

    #[arg(short = 'c')]
    /// Whether the todo item has been completed
    pub completed: bool,
}
