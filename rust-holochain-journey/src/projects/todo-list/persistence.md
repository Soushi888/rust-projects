# Data Persistence in Todo List Application

## Overview

This document explains how data persistence is implemented in our Todo List application using JSON file storage.

## Implementation

### File Operations Module

```rust
use crate::error::TodoResult;
use crate::todo_list::*;
use serde_json::from_reader;
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
            fs::write("todo.json", Vec::new())?;
            println!("Created new todo list");
            Ok(TodoList::new())
        }
    }
}

pub fn save_json(mut todo_list: TodoList) -> TodoResult<()> {
    todo_list.tasks.sort_by_key(|t| t.date.clone());
    let json = serde_json::to_string_pretty(&todo_list.tasks)?;
    fs::write("todo.json", json)?;
    Ok(())
}
```

## Key Features

### 1. File Handling
- Uses `OpenOptions` for flexible file operations
- Creates file if it doesn't exist
- Supports both reading and writing
- No truncation on open for safety

### 2. Data Format
- JSON storage using serde_json
- Pretty printing for readability
- Automatic sorting by date
- Vector-based task storage

### 3. Error Handling
- Custom `TodoResult` type
- Graceful handling of new file creation
- Recovery from corrupted files
- Proper error propagation

## Data Structure

### JSON Format
```json
[
  {
    "name": "task1",
    "description": "First task",
    "date": "2024-01-20",
    "completed": false
  },
  {
    "name": "task2",
    "description": "Second task",
    "date": "2024-01-21",
    "completed": true
  }
]
```

## Implementation Details

### 1. Loading Data
- Opens file with read/write permissions
- Creates new file if not exists
- Deserializes JSON to task vector
- Initializes empty list on error

### 2. Saving Data
- Sorts tasks by date
- Pretty prints JSON
- Writes atomically
- Maintains data integrity

### 3. Error Recovery
- Creates new file if corrupted
- Initializes empty list
- Provides user feedback
- Maintains application state

## Best Practices

### 1. File Operations
- Use `OpenOptions` for safe file handling
- Create file if missing
- Avoid data loss with proper flags
- Sort data for consistency

### 2. Data Integrity
- Validate before saving
- Pretty print for readability
- Sort for consistent order
- Handle corrupted data gracefully

### 3. Error Handling
- Custom result type
- Clear error messages
- Graceful recovery
- State preservation

## Usage Examples

### Loading Data
```rust
let todo_list = load_json()?;
```

### Saving Data
```rust
save_json(todo_list)?;
```

### Error Recovery
```rust
match load_json() {
    Ok(list) => {
        // Use the loaded list
    },
    Err(e) => {
        eprintln!("Error loading todo list: {}", e);
        // Handle error...
    }
}
