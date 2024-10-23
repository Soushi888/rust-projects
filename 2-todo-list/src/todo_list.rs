use std::process;

use crate::{
  cli::UpdateTaskArgs,
  error::{TodoError, TodoResult},
  task::Task,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoList {
  pub tasks: Vec<Task>,
}

impl TodoList {
  pub fn new() -> Self {
    Self { tasks: Vec::new() }
  }

  pub fn add(&mut self, mut task: Task) -> TodoResult<()> {
    let now = chrono::Local::now().format("%Y-%m-%d").to_string();
    if task.date.is_empty() {
      task.date = now;
    }

    if self.find(&task.name).is_some() {
      return Err(TodoError::DuplicateTask);
    }

    self.tasks.push(task.clone());
    println!("Task \"{}\" added", task.name);
    Ok(())
  }

  pub fn update(&mut self, task_name: &str, updated_task: UpdateTaskArgs) -> TodoResult<Self> {
    if updated_task.new_name.is_none()
      && updated_task.description.is_none()
      && updated_task.date.is_none()
      && updated_task.completed.is_none()
    {
      return Err(TodoError::InvalidTaskState);
    }

    let task = self.find(task_name).ok_or(TodoError::TaskNotFound)?;

    let updated_task = task.to_owned();

    self.retain(task_name);
    self.tasks.push(updated_task);
    println!("Task \"{}\" updated", task_name);

    Ok(self.to_owned())
  }

  pub fn complete(&mut self, task_name: &str) -> TodoResult<()> {
    let mut task = self
      .find(task_name)
      .ok_or(TodoError::TaskNotFound)?
      .to_owned();
    task.completed = true;

    self.retain(&task.name);
    self.push(task.clone());
    println!("Task \"{}\" completed", task.name);

    Ok(())
  }

  pub fn uncomplete(&mut self, task_name: &str) -> TodoResult<()> {
    let mut task = self
      .find(task_name)
      .ok_or(TodoError::TaskNotFound)?
      .to_owned();
    task.completed = false;

    self.retain(&task.name);
    self.push(task.clone());
    println!("Task \"{}\" uncompleted", task.name);

    Ok(())
  }

  pub fn remove(&mut self, task_name: &str) -> TodoResult<()> {
    let list_len = self.tasks.len();
    self.retain(task_name);
    let is_removed = list_len != self.tasks.len();

    if !is_removed {
      return Err(TodoError::TaskNotFound);
    };

    println!("Task \"{}\" removed", task_name);
    Ok(())
  }

  pub fn list(&self) {
    if self.tasks.is_empty() {
      println!("No tasks found");
      return;
    }
    for task in &self.tasks {
      println!("{}", task);
    }
  }

  pub fn find(&self, name: &str) -> Option<&Task> {
    self.tasks.iter().find(|t| t.name == *name)
  }

  pub fn retain(&mut self, name: &str) {
    self.tasks.retain(|t| t.name != *name);
  }

  pub fn push(&mut self, task: Task) {
    self.tasks.push(task);
  }

  pub fn clear_completed(&mut self) {
    self.tasks.retain(|t| !t.completed);
    println!("Completed tasks cleared");
  }

  pub fn clear_all(&mut self) {
    self.tasks.clear();
    println!("Todo list cleared");
  }
}
