# Todo List Project Requirements

## Core Features

### Task Management
- Create new tasks with name, description, and due date
- View all tasks in the list
- Update existing task details
- Remove tasks from the list
- Mark tasks as complete/incomplete
- View task status

### Data Operations
- Clear all completed tasks
- Clear entire todo list
- Persist tasks to JSON file
- Load tasks from JSON file

## Implementation Details

### Command Line Interface
```bash
# View all tasks
todo view

# Add a new task
todo add <name> [--description <desc>] [--date <date>]

# Remove a task
todo remove <name>

# Update a task
todo update <name> [--new-name <name>] [--description <desc>] [--date <date>] [--completed <bool>]

# Complete/Uncomplete tasks
todo complete <name>
todo uncomplete <name>

# View task status
todo status <name>

# Clear operations
todo clear-completed
todo clear-all
```

### Data Structure

#### Task
```rust
pub struct Task {
    pub name: String,
    pub description: String,
    pub date: String,
    pub completed: bool,
}
```

#### TodoList
```rust
pub struct TodoList {
    pub tasks: Vec<Task>,
}
```

### Error Types
```rust
pub enum TodoError {
    TaskNotFound,
    DuplicateTask,
    InvalidTaskState,
}

pub type TodoResult<T> = Result<T, TodoError>;
```

## Dependencies

### Required Crates
```toml
[dependencies]
clap = { version = "4.4.11", features = ["derive"] }
serde = { version = "1.0.193", features = ["derive"] }
serde_json = "1.0.108"
chrono = "0.4.31"
```

## Project Structure
```
src/
├── main.rs       # Application entry point
├── cli.rs        # CLI interface definitions
├── error.rs      # Custom error types
├── file.rs       # File I/O operations
├── task.rs       # Task data structure
└── todo_list.rs  # Core todo list logic
```

## Features Implementation

### Task Display
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

### Task Operations
- Creation with optional fields
- Status display
- Completion toggling
- Update with partial fields

### List Operations
- Add tasks with duplicate checking
- Remove tasks by name
- Update tasks with validation
- List all tasks with formatting
- Clear completed or all tasks

### Data Persistence
- JSON file storage
- Automatic date handling
- Error recovery
- Pretty printing
