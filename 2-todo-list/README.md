# Rust Todo List CLI Application

A command-line todo list application written in Rust that allows you to manage your tasks with persistent storage using JSON.

## Features

- Add new tasks with name, description, and due date
- View all tasks in the list
- Remove tasks by name
- Update existing tasks
- Mark tasks as complete
- Persistent storage using JSON file

## Installation

Make sure you have Rust installed on your system. Then clone this repository and build the project:

```bash
git clone <repository-url>
cd 2-todo-list
cargo build --release
```

The executable will be available in `target/release/todo-list`

## Usage

The application supports the following commands:

### View all tasks
```bash
todo-list view
```

### Add a new task
```bash
todo-list add <task-name> [--description <description>] [--date <due-date>]
```

### Remove a task
```bash
todo-list remove <task-name>
```

### Update a task
```bash
todo-list update <task-name> [--description <new-description>] [--date <new-due-date>]
```

### Mark a task as complete
```bash
todo-list complete <task-name>
```

## Project Structure

- `src/main.rs`: Main application entry point and command handling
- `src/cli.rs`: Command-line interface definitions using clap
- `src/task.rs`: Task data structure and related functionality
- `src/todo_list.rs`: Todo list management logic
- `src/file.rs`: File operations for JSON persistence
- `src/error.rs`: Custom error handling

## Dependencies

- clap (4.5): Command-line argument parsing
- chrono (0.4): Date and time functionality
- serde (1.0): Serialization framework
- serde_json (1.0): JSON serialization/deserialization

## License

This project is open source and available under the MIT License.
