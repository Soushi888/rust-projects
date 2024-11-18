# Rust Exercise: Library Management System

## Overview
Create a command-line library management system that allows users to manage books and track borrowing records. This exercise will help consolidate your understanding of Rust fundamentals while introducing some new concepts.

## Requirements

### 1. Data Structures
```rust
// Implement these structs with appropriate fields and methods
struct Book {
    title: String,
    author: String,
    isbn: String,
    year: u32,
    copies_available: u32,
}

struct BorrowRecord {
    isbn: String,
    borrower: String,
    borrow_date: DateTime<Utc>,
    return_date: Option<DateTime<Utc>>,
}

struct Library {
    books: Vec<Book>,
    borrow_records: Vec<BorrowRecord>,
}
```

### 2. Required Traits
- [ ] Implement `Display` for Book to pretty-print book information
- [ ] Implement `Debug` for all structs
- [ ] Implement custom error types for the library operations

### 3. Command-Line Interface
Implement the following commands using Clap:

```bash
# Adding a new book
library add <title> --author <author> --isbn <isbn> --year <year> --copies <number>

# Listing all books
library list

# Searching books
library search <query>  # Should search in both title and author

# Borrowing operations
library checkout <isbn> <borrower>
library return <isbn>

# View history
library history <isbn>  # Shows all borrowing records for a book
```

### 4. File Operations
- [ ] Implement JSON storage for books and borrowing records
- [ ] Handle file read/write errors appropriately
- [ ] Implement automatic saving after each operation

### 5. Error Handling
Create custom errors for:
- [ ] Book not found
- [ ] ISBN already exists
- [ ] No copies available
- [ ] Invalid ISBN format
- [ ] File operation errors

## Project Structure
```
src/
├── main.rs
├── cli.rs      # Command-line interface definitions
├── book.rs     # Book struct and implementations
├── borrow.rs   # BorrowRecord struct and implementations
├── library.rs  # Library struct and main logic
├── error.rs    # Custom error types
└── file.rs     # File operations
```

## Getting Started
1. Create a new Rust project:
```bash
cargo new library-management
cd library-management
```

2. Add required dependencies to Cargo.toml:
```toml
[package]
name = "library-management"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

## Tips
1. Start with basic book management before implementing borrowing functionality
2. Use the `Option` type for fields that might not have values
3. Implement proper validation for ISBN and other fields
4. Keep error messages user-friendly
5. Test each component separately before integrating

## Submission
Your submission should include:
1. Complete source code
2. README file with:
   - Setup instructions
   - Usage examples
   - Design decisions explanation
3. Sample data files (if any)

## Timeline
Recommended time allocation:
- Basic setup and structure: 1 hour
- Core functionality: 2-3 hours
- Error handling: 1-2 hours
- Testing and refinement: 1 hour
- Documentation: 1 hour

Total estimated time: 6-8 hours
