use std::process;

use crate::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoList {
  pub tasks: Vec<Task>,
}

impl TodoList {
  pub fn new() -> Self {
    Self { tasks: Vec::new() }
  }

  pub fn add(&mut self, mut task: Task) {
    let now = chrono::Local::now().format("%Y-%m-%d").to_string();
    if task.date.is_empty() {
      task.date = now;
    }

    if self.find(&task.name).is_some() {
      println!("Task \"{}\" already exists", task.name);
      process::exit(1);
    }

    self.push(task.clone());
    println!("Task \"{}\" added", task.name);
  }

  pub fn update(&mut self, task: Task, updated_task: Task) {
    self.retain(&task.name);
    self.tasks.push(updated_task);
    println!("Task \"{}\" updated", task.name);
  }

  pub fn complete(&mut self, mut task: Task) {
    task.completed = true;

    self.retain(&task.name);
    self.push(task.clone());
    println!("Task \"{}\" completed", task.name);
  }

  pub fn uncomplete(&mut self, mut task: Task) {
    task.completed = false;

    self.retain(&task.name);
    self.push(task.clone());
    println!("Task \"{}\" uncompleted", task.name);
  }

  pub fn remove(&mut self, task: Task) {
    let list_len = self.tasks.len();
    self.retain(&task.name);
    let is_removed = list_len != self.tasks.len();

    if is_removed {
      println!("Task \"{}\" removed", task.name);
    } else {
      println!("Task \"{}\" not found", task.name);
      process::exit(1);
    }
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
