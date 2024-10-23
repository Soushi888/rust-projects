use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author = "Soushi888", version)]
/// Simple todo cli app
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
  /// View all tasks
  View,
  /// Add a new task
  Add(AddTaskArgs),
  Remove(TaskNameArg),
  /// Update a task
  Update(UpdateTaskArgs),
  /// Complete a task
  Complete(TaskNameArg),
  /// Uncomplete a task
  Uncomplete(TaskNameArg),
  /// Show the status of the task
  Status(TaskNameArg),
  /// Clear completed tasks
  ClearCompleted,
  /// Clear all tasks
  ClearAll,
}

#[derive(Args, Debug, Clone)]
pub struct AddTaskArgs {
  /// Task name
  pub name: String,
  /// Task description
  pub description: Option<String>,
  /// Task date
  pub date: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct TaskNameArg {
  /// Task name
  pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct UpdateTaskArgs {
  pub name: String,
  #[clap(long, short)]
  pub new_name: Option<String>,
  #[clap(long, short)]
  pub description: Option<String>,
  #[clap(long)]
  pub date: Option<String>,
  #[clap(long, short)]
  pub completed: Option<bool>,
}
