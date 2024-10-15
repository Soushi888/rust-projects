use crate::task::*;
use crate::todo_list::*;
use serde_json::{from_reader, to_writer};
use std::fs;
use std::fs::OpenOptions;
use std::io::Error;

pub fn load_json() -> Result<TodoList, Error> {
  let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("todo.json")?;

  if let Ok(tasks) = from_reader(&file) {
    Ok(TodoList { tasks })
  } else {
    to_writer(file, &Vec::<Task>::new())?;
    println!("Created new todo list");
    Ok(TodoList::new())
  }
}

pub fn save_json(mut todo_list: Vec<Task>) -> Result<(), Error> {
  todo_list.sort_by_key(|t| t.date.clone());
  let json = serde_json::to_string_pretty(&todo_list).unwrap();
  fs::write("todo.json", json)?;

  Ok(())
}
