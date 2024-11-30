# Minimalist CLI Design and Implementation

## Introduction

This section provides a straightforward guide to designing and implementing a basic CLI application in Rust. The goal is to introduce fundamental concepts and patterns that are essential for creating functional and efficient command-line tools.

## Key Concepts

- **Simplicity**: Focus on essential features that demonstrate core Rust capabilities.
- **Modularity**: Break down the application into small, manageable components.
- **Error Handling**: Use Rust's powerful error handling mechanisms to create robust applications.

## Project Structure

The project consists of a single `main.rs` file that implements a simple file reader CLI tool. Here's the breakdown of its components:

### Core Components

1. **Main Function**
   - Handles command-line arguments
   - Implements an interactive loop for reading multiple files
   - Uses proper error handling with `io::Result`

2. **File Reading Function**
   - Encapsulated file reading logic
   - Uses Rust's standard I/O operations
   - Implements error handling

## Implementation Details

### Command-Line Arguments

```rust
let args: Vec<String> = std::env::args().collect();
if args.len() < 2 {
    eprintln!("Usage: {} <filename>", args[0]);
    return Ok(());
}
let filename = &args[1];
```

### File Reading Implementation

```rust
/// Reads the contents of a file and returns them as a string.
///
/// ## Errors
///
/// Returns an error if there was a problem opening or reading the file.
fn read_file(filename: &str) -> io::Result<String> {
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}
```

### Interactive Loop

```rust
loop {
    print!("Do you want to read another file? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
        "y" => {
            print!("Enter filename: ");
            io::stdout().flush()?;

            let mut filename = String::new();
            io::stdin().read_line(&mut filename)?;
            let filename = filename.trim();

            let content = read_file(filename)?;
            println!("{}", content);
        }
        "n" | "" => {
            println!("Bye!");
            break;
        }
        _ => println!("Invalid input. Please enter y or n."),
    }
}
```

## Key Rust Features Demonstrated

1. **Error Handling**
   - Use of `Result` type
   - The `?` operator for error propagation
   - Proper error messages

2. **Input/Output**
   - File operations with `std::fs`
   - Standard input/output handling
   - Buffered reading for efficiency

3. **Control Flow**
   - Pattern matching with `match`
   - Loop control with `break`
   - Early returns

4. **String Manipulation**
   - String creation and modification
   - String trimming and case conversion
   - String ownership and borrowing

## Testing the Implementation

To run the program:

```bash
cargo run -- example.txt
```

The program will:
1. Read and display the content of example.txt
2. Ask if you want to read another file
3. Continue based on your input (y/n)

## Conclusion

By following these steps, you'll gain a solid understanding of how to build simple yet effective CLI applications in Rust. This foundational knowledge will prepare you for more complex projects as you continue your Rust journey.
