# State Management in Todo List Application

## Overview

This document explains how state is managed in our Todo List application, demonstrating Rust's powerful ownership and borrowing system.

## Core State Structure

### Task Structure
```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub date: String,
    pub completed: bool,
}
```

### TodoList Structure
```rust
#[derive(Debug, Clone)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}
```

## State Management Patterns

### 1. Centralized State
- Single `TodoList` instance maintains all tasks
- Tasks are stored in a `Vec` for ordered storage
- Task names serve as unique identifiers

### 2. Task Management
```rust
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
```

### 3. State Modifications
```rust
impl TodoList {
    // Add new task with automatic date handling
    pub fn add(&mut self, mut task: Task) -> TodoResult<()> {
        let now = Local::now().format("%Y-%m-%d").to_string();
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

    // Update existing task
    pub fn update(&mut self, task_name: &str, updated_task: UpdateTaskArgs) -> TodoResult<Self> {
        // Validation
        if updated_task.new_name.is_none()
            && updated_task.description.is_none()
            && updated_task.date.is_none()
            && updated_task.completed.is_none()
        {
            return Err(TodoError::InvalidTaskState);
        }

        let task = self.find(task_name).ok_or(TodoError::TaskNotFound)?;
        // Update task...
        Ok(self.to_owned())
    }
}
```

## Error Handling

### 1. Custom Error Types
```rust
pub enum TodoError {
    TaskNotFound,
    DuplicateTask,
    InvalidTaskState,
    // ... other error types
}
```

### 2. Result Type
```rust
pub type TodoResult<T> = Result<T, TodoError>;
```

## Task Display

The Task implements Display for formatted output:
```rust
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
```

## Best Practices

1. **State Consistency**
   - Validate task existence before modifications
   - Check for duplicate tasks
   - Ensure all updates maintain data integrity

2. **Error Handling**
   - Use custom error types for specific scenarios
   - Implement proper error propagation
   - Provide meaningful error messages

3. **Data Validation**
   - Validate task updates
   - Handle empty or invalid dates
   - Prevent duplicate task names

4. **Memory Management**
   - Use Clone trait where necessary
   - Implement proper ownership transfer
   - Use references for read-only operations
