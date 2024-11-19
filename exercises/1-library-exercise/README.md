# Library Management System: Rust Advanced Exercise

## Overview
A comprehensive Rust programming exercise demonstrating advanced language features through a practical library management system.

## Quick Links
- [Detailed Specification](docs/SPECIFICATION.md)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)

## Learning Objectives
- Master advanced Rust programming techniques
- Build a real-world application
- Practice software design principles

## Key Concepts
- Complex struct design
- Advanced error handling
- CLI application development
- Data persistence
- Trait implementations

## Project Structure
```
library-exercise/
├── Cargo.toml           # Project dependencies
├── src/                 # Source code
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command-line interface
│   ├── book.rs          # Book-related structs
│   ├── library.rs       # Library management logic
│   ├── errors.rs        # Custom error types
│   └── storage.rs       # File storage operations
├── docs/
│   └── SPECIFICATION.md # Detailed project specification
└── README.md            # Project documentation
```

## Prerequisites
- Rust programming language (latest stable version)
- Cargo package manager
- Basic understanding of Rust concepts

## Getting Started
1. Review the [detailed specification](docs/SPECIFICATION.md)
2. Clone the repository
3. Install dependencies: `cargo build`
4. Run the application: `cargo run`
5. Execute tests: `cargo test`

## CLI Commands
```bash
# Add a new book
library add "Rust Programming" --author "Steve Klabnik" --isbn "1234567890" --year 2021

# List all books
library list

# Search books
library search "Rust"

# Checkout a book
library checkout 1234567890 "John Doe"

# Return a book
library return 1234567890
```

## Recommended Resources
- [Rust Programming Book](https://doc.rust-lang.org/book/)
- [Clap CLI Documentation](https://docs.rs/clap/)
- [Serde Serialization](https://serde.rs/)

## Challenges and Extensions
- Implement advanced search features
- Add user authentication
- Create a web interface
- Integrate with a database

## Contributing
Improvements and suggestions are welcome!
- Open an issue
- Submit a pull request
- Provide constructive feedback

## License
[Specify your license]
