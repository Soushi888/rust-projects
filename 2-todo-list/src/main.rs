mod file;
mod task;
mod todo_list;

use crate::file::*;
use crate::task::*;
use clap::{Args, Parser, Subcommand};
use std::process;

#[derive(Debug, Parser)]
#[clap(author = "Soushi888", version)]
/// Simple todo cli app
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
  /// View all tasks
  View,
  /// Add a new task
  Add(AddTaskArgs),
  /// Remove a task
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
struct AddTaskArgs {
  /// Task name
  name: String,
  /// Task description
  description: Option<String>,
  /// Task date
  date: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct TaskNameArg {
  /// Task name
  name: String,
}

#[derive(Args, Debug, Clone)]
struct UpdateTaskArgs {
  name: String,
  #[clap(long, short)]
  new_name: Option<String>,
  #[clap(long, short)]
  description: Option<String>,
  #[clap(long)]
  date: Option<String>,
  #[clap(long, short)]
  completed: Option<bool>,
}

use Commands::*;

fn main() {
  let cli = Cli::parse();

  let mut todo_list = if let Ok(todo_list) = load_json() {
    todo_list
  } else {
    process::exit(1);
  };

  match cli.command {
    View => {
      todo_list.list();
    }
    Add(args) => {
      let task = Task::new(
        args.name,
        args.description.unwrap_or_default(),
        args.date.unwrap_or_default(),
        None,
      );
      todo_list.add(task);
      save_json(todo_list.tasks).unwrap();
    }
    Remove(args) => {
      let task = Task::new(args.name, String::new(), String::new(), None);
      todo_list.remove(task);
      save_json(todo_list.tasks).unwrap();
    }
    Update(args) => {
      if args.new_name.is_none()
        && args.description.is_none()
        && args.date.is_none()
        && args.completed.is_none()
      {
        println!("No value to update");
        process::exit(1);
      }

      let task = todo_list.find(&args.name).unwrap();
      let new_task = Task::new(
        args.new_name.unwrap_or(args.name),
        args.description.unwrap_or(task.description.clone()),
        args.date.unwrap_or(task.date.clone()),
        Some(args.completed.unwrap_or(task.completed)),
      );
      todo_list.update(task.to_owned(), new_task);
      save_json(todo_list.tasks).unwrap();
    }
    Complete(args) => {
      let task = todo_list.find(&args.name);
      if task.is_none() {
        println!("Task \"{}\" not found", args.name);
        process::exit(1);
      }

      todo_list.complete(task.unwrap().to_owned());
      save_json(todo_list.tasks).unwrap();
    }
    Uncomplete(args) => {
      let task = todo_list.find(&args.name);
      if task.is_none() {
        println!("Task \"{}\" not found", args.name);
        process::exit(1);
      }
      todo_list.uncomplete(task.unwrap().to_owned());
      save_json(todo_list.tasks).unwrap();
    }
    Status(args) => {
      let task = todo_list.find(&args.name);
      if task.is_none() {
        println!("Task \"{}\" not found", args.name);
        process::exit(1);
      }
      task.unwrap().status();
    }
    ClearCompleted => {
      todo_list.clear_completed();
      save_json(todo_list.tasks).unwrap();
    }
    ClearAll => {
      todo_list.clear_all();
      save_json(todo_list.tasks).unwrap();
    }
  }
}
