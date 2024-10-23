mod cli;
mod error;
mod file;
mod task;
mod todo_list;

use crate::file::*;
use crate::task::*;
use std::process;

use clap::Parser;
use cli::{Cli, Commands::*};
use error::TodoError;
use error::TodoResult;

fn main() -> TodoResult<()> {
  let cli = Cli::parse();

  let mut todo_list = load_json()?;

  match cli.command {
    View => {
      todo_list.list();
      Ok(())
    }
    Add(args) => {
      let task = Task::new(
        args.name,
        args.description.unwrap_or_default(),
        args.date.unwrap_or_default(),
        None,
      );

      todo_list.add(task)?;
      save_json(todo_list.tasks.clone())?;

      Ok(())
    }
    Remove(args) => {
      todo_list.remove(&args.name)?;
      save_json(todo_list.tasks)?;

      Ok(())
    }
    Update(args) => {
      todo_list.update(&args.name.clone(), args)?;
      save_json(todo_list.tasks.clone())?;

      Ok(())
    }
    Complete(args) => {
      todo_list.complete(&args.name)?;
      save_json(todo_list.tasks)?;

      Ok(())
    }
    Uncomplete(args) => {
      todo_list.uncomplete(&args.name)?;
      save_json(todo_list.tasks)?;

      Ok(())
    }
    Status(args) => {
      let task = todo_list.find(&args.name).ok_or(TodoError::TaskNotFound)?;
      task.status();

      Ok(())
    }
    ClearCompleted => {
      todo_list.clear_completed();
      save_json(todo_list.tasks)?;

      Ok(())
    }
    ClearAll => {
      todo_list.clear_all();
      save_json(todo_list.tasks)?;

      Ok(())
    }
  }
}
