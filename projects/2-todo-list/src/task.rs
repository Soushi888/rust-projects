use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
  pub name: String,
  pub description: String,
  pub date: String,
  pub completed: bool,
}

impl Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let completed = if self.completed { "x" } else { " " };
    write!(
      f,
      "[{}] {} - {} - {}",
      completed, self.name, self.description, self.date
    )
  }
}

impl Task {
  pub fn new(name: String, description: String, date: String, completed: Option<bool>) -> Self {
    Self {
      name,
      description,
      date,
      completed: completed.unwrap_or(false),
    }
  }

  pub fn status(&self) {
    let status = if self.completed {
      "completed"
    } else {
      "uncompleted"
    };
    println!("Task \"{}\" is {}", self.name, status);
  }
}
