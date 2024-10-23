// TODO: implement thiserror crate
// https://crates.io/crates/thiserror

use std::{error, fmt, io};

pub type TodoResult<T> = Result<T, TodoError>;

// Define our custom error type
pub enum TodoError {
  Io(io::Error),           // IO error
  Json(serde_json::Error), // JSON serialization/deserialization error
  TaskNotFound,            // Task not found
  InvalidTaskState,        // Invalid task state
  DuplicateTask,           // Duplicate task
}

impl fmt::Display for TodoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TodoError::Io(msg) => write!(f, "IO error: {}", msg),
      TodoError::Json(e) => write!(f, "JSON serialization/deserialization error: {}", e),
      TodoError::TaskNotFound => write!(f, "Task not found"),
      TodoError::InvalidTaskState => write!(f, "Invalid task state"),
      TodoError::DuplicateTask => write!(f, "Duplicate task"),
    }
  }
}

impl fmt::Debug for TodoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

// Implement Error trait for our custom error type
impl error::Error for TodoError {}

// Implement From trait for IO errors
impl From<std::io::Error> for TodoError {
  fn from(e: std::io::Error) -> Self {
    TodoError::Io(e)
  }
}

// Implement From trait for JSON errors
impl From<serde_json::Error> for TodoError {
  fn from(e: serde_json::Error) -> Self {
    TodoError::Json(e)
  }
}
