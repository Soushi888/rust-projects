use crate::error::TodoResult;
use crate::task::*;
use crate::todo_list::*;
use serde_json::{from_reader, to_writer};
use std::fs;
use std::fs::OpenOptions;

pub fn load_json() -> TodoResult<TodoList> {
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .truncate(false)
    .create(true)
    .open("todo.json")?;

  match from_reader(&file) {
    Ok(tasks) => Ok(TodoList { tasks }),
    Err(_) => {
      to_writer(file, &Vec::<Task>::new())?;
      println!("Created new todo list");
      Ok(TodoList::new())
    }
  }
}

pub fn save_json(mut todo_list: Vec<Task>) -> TodoResult<()> {
  todo_list.sort_by_key(|t| t.date.clone());
  let json = serde_json::to_string_pretty(&todo_list)?;
  fs::write("todo.json", json)?;

  Ok(())
}
