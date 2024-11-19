# Basic File Reader CLI

## Project Overview

In this project, we'll create a simple command-line interface (CLI) application in Rust that allows users to read file contents interactively. This project will introduce several key Rust concepts and standard library features.

## Learning Objectives

By completing this project, you will learn:
- Command-line argument parsing
- File I/O operations
- Error handling with `Result` type
- User input handling
- Basic control flow in Rust

## Project Structure

```
1-basic-cli/
├── Cargo.toml
└── src/
    └── main.rs
```

## Step-by-Step Implementation

### 1. Project Setup

Create a new Rust project using Cargo:
```bash
cargo new basic-cli
cd basic-cli
```

For more on setting up and managing Rust projects, see [Language Basics - Imports](../../rust/language-basics.md#imports).

### 2. Implementing the File Reader

Open `src/main.rs` and follow these steps to implement the file reader:

#### a. Import Necessary Modules

Start by importing the necessary modules for file I/O and user interaction:

```rust
use std::fs::File; // Import the File type for handling file operations
use std::io::{self, Read, Write}; // Import I/O traits for reading, writing, and flushing
```

Refer to [Language Basics - Imports](../../rust/language-basics.md#imports) for more details on using imports in Rust.

#### b. Define the Main Function

Create the `main` function, which will be the entry point of the application:

```rust
fn main() -> io::Result<()> { // Main function returning a Result type for error handling
```

Learn more about functions in [Language Basics - Functions](../../rust/language-basics.md#functions).

#### c. Handle Command-Line Arguments

Collect command-line arguments and ensure a filename is provided:

```rust
let args: Vec<String> = std::env::args().collect(); // Collect command-line arguments into a vector
if args.len() < 2 { // Check if a filename is provided
    eprintln!("Usage: {} <filename>", args[0]); // Print usage message to standard error
    return Ok(()); // Return early if no filename is provided
}
```

#### d. Read and Display File Content

Call the `read_file` function to read the specified file and display its contents:

```rust
let filename = &args[1]; // Get the filename from the command-line arguments
let content = read_file(filename)?; // Read the file content, propagating errors
println!("{}", content); // Print the file content to the console
```

#### e. Implement Interactive Loop

Create a loop to allow the user to read additional files interactively:

```rust
loop { // Start an infinite loop for user interaction
    print!("Do you want to read another file? (y/n): "); // Prompt the user
    io::stdout().flush()?; // Flush stdout to ensure the prompt is displayed

    let mut input = String::new(); // Create a mutable string to store user input
    io::stdin().read_line(&mut input)?; // Read a line of input from the user

    match input.trim().to_lowercase().as_str() { // Match the trimmed, lowercase input
        "y" => {
            print!("Enter filename: "); // Prompt for a new filename
            io::stdout().flush()?; // Flush stdout

            let mut filename = String::new(); // Create a mutable string for the filename
            io::stdin().read_line(&mut filename)?; // Read the filename from user input
            let filename = filename.trim(); // Trim whitespace from the filename

            let content = read_file(filename)?; // Read and print the file content
            println!("{}", content);
        }
        "n" | "" => { // Exit the loop if the user inputs 'n' or nothing
            println!("Bye!");
            break;
        }
        _ => println!("Invalid input. Please enter y or n."), // Handle invalid input
    }
}
```

For more on loops, see [Language Basics - Loops](../../rust/language-basics.md#loops).

#### f. Define the File Reading Function

Implement the `read_file` function to handle file reading:

```rust
fn read_file(filename: &str) -> io::Result<String> { // Function to read file content
    let mut file = File::open(filename)?; // Open the file, propagating errors
    let mut contents = String::new(); // Create a string to hold file contents
    file.read_to_string(&mut contents)?; // Read the file into the string
    Ok(contents) // Return the file contents
}
```

### 3. Key Rust Concepts Explained

#### Command-line Arguments
- `std::env::args()` returns an iterator of command-line arguments
- `collect()` converts the iterator into a `Vec<String>`

#### Error Handling
- `io::Result<()>` allows propagating I/O errors
- `?` operator for concise error handling
- `eprintln!()` for printing to standard error
For more on error handling, see [Error Handling - Result and Option](../../rust/error-handling.md).

#### File I/O
- `File::open()` opens a file
- `read_to_string()` reads entire file contents
- Error handling with `?` operator

#### User Input
- `io::stdin().read_line()` reads user input
- `io::stdout().flush()` ensures prompt is displayed
- `match` for handling different input scenarios

### 4. Running the Project

```bash
# Run with a file argument
cargo run -- example.txt

# Build for release
cargo build --release
```

## Challenges and Extensions

1. Add file size display before reading
2. Implement error handling for non-existent files
3. Add support for reading multiple files
4. Create a configuration to set default file paths

## Best Practices

- Use `Result` for error handling
- Keep functions small and focused
- Use descriptive variable names
- Handle edge cases (e.g., no arguments)

## Conclusion

This basic CLI project demonstrates Rust's strengths in systems programming: safe error handling, powerful standard library, and expressive syntax.
