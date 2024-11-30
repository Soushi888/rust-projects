# Error Handling in Rust

## Introduction
When writing programs, things don't always go as planned. A file might be missing, a network connection might fail, or a user might enter invalid data. Error handling is how we deal with these problems gracefully.

Rust has two main ways of handling errors:
1. `Result<T, E>` for recoverable errors (things we can fix or handle)
2. `panic!` for unrecoverable errors (things so bad we need to stop the program)

Think of it like this:
- `Result` is like having a backup plan when something goes wrong
- `panic!` is like hitting the emergency stop button when something is seriously wrong

## 1. Panic and Unrecoverable Errors

### What is a Panic?
A panic is Rust's way of saying "Something is so wrong that we can't continue safely." When a panic happens, your program will:
1. Print an error message
2. Clean up program resources
3. Exit

### When to Use panic!
Use `panic!` when:
- The program reaches an impossible state (like dividing by zero)
- You're writing example code and want to show errors quickly
- You're in a situation where continuing would be dangerous

```rust
fn main() {
    // Example 1: Explicit panic
    panic!("Something went terribly wrong!");  // Program stops here with message

    // Example 2: Implicit panic (array access)
    let numbers = vec![1, 2, 3];
    let item = numbers[99];  // This will panic - index way too large!
}
```

## 2. Result Type for Recoverable Errors

### What is Result?
`Result` is like a package that can contain one of two things:
- `Ok(value)` - "Everything worked! Here's your value"
- `Err(error)` - "Something went wrong! Here's what happened"

### Basic Result Usage
```rust
// Result's basic structure
enum Result<T, E> {
    Ok(T),    // T is the type of success value
    Err(E),   // E is the type of error value
}

// A function that might fail
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        // Can't divide by zero! Return an error
        Err(String::from("division by zero"))
    } else {
        // Division worked! Return the result
        Ok(x / y)
    }
}

// Using our divide function
let result = divide(10.0, 2.0);
match result {
    Ok(value) => println!("Success! Result is {}", value),
    Err(e) => println!("Oops! Error: {}", e),
}
```

### Result Combinators - Making Error Handling Easier
Instead of using match every time, Result has helpful methods:

```rust
// map_err: Change the error type
let result = divide(10.0, 0.0)
    .map_err(|e| format!("Math failed: {}", e));  // Add more context to error

// and_then: Chain operations that might fail
let result = divide(10.0, 2.0)          // First division
    .and_then(|n| divide(n, 2.0));      // Use result in second division

// unwrap_or: Provide a default value if there's an error
let value = divide(10.0, 0.0)
    .unwrap_or(0.0);  // Use 0.0 if division fails

// expect: Like unwrap but with a custom panic message
let value = divide(10.0, 2.0)
    .expect("Basic division should work!");
```

## 3. The ? Operator - Making Error Handling Beautiful

### What is the ? Operator?
The ? operator is a shortcut for handling Results. It means:
- If Ok, give me the value inside
- If Err, return the error from this function immediately

### Basic Usage
```rust
use std::fs::File;
use std::io::{self, Read};

// Without ? operator (verbose)
fn read_file_verbose() -> Result<String, io::Error> {
    let file_result = File::open("config.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

// With ? operator (clean!)
fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("config.txt")?;  // ? handles the error case
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;       // ? handles the error case
    Ok(contents)
}
```

### Chaining Operations
The ? operator makes it easy to chain multiple operations that might fail:

```rust
fn read_username() -> Result<String, io::Error> {
    // All of this happens in one line!
    let mut username = String::new();
    File::open("username.txt")?         // Try to open file
        .read_to_string(&mut username)?; // Try to read to string
    Ok(username)
}
```

## 4. Custom Error Types - Making Your Own Errors

### Why Custom Error Types?
When writing larger programs, you often want to:
1. Have specific error types for different problems
2. Add extra information to your errors
3. Convert between different error types

### Creating a Custom Error Type
```rust
// Define our custom error type
#[derive(Debug)]  // Let us print the error for debugging
pub enum AppError {
    IoError(std::io::Error),         // For file/network problems
    ParseError(std::num::ParseIntError), // For number parsing problems
    Custom(String),                  // For our own error messages
}

// Make it work like a standard error
impl std::error::Error for AppError {}

// Make it easy to display
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "File error: {}", e),
            AppError::ParseError(e) => write!(f, "Couldn't parse number: {}", e),
            AppError::Custom(s) => write!(f, "{}", s),
        }
    }
}

// Make it easy to convert from other error types
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}
```

### Using Custom Error Types
```rust
use std::fs::File;
use std::io::Read;

fn read_and_parse() -> Result<i32, AppError> {
    // Open and read file
    let mut file = File::open("number.txt")?;  // IoError converts automatically
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;       // IoError converts automatically
    
    // Parse string to number
    let number: i32 = contents.trim().parse()? // ParseError converts automatically
    Ok(number)
}
```

## 5. Best Practices - Writing Good Error Handling

### Error Handling Guidelines
1. Use `Result` for errors that can be handled
2. Use `panic!` only for unrecoverable errors
3. Create custom error types for your libraries
4. Give helpful error messages
5. Use the ? operator to keep code clean
6. Convert between error types when needed

### Common Patterns
```rust
// Pattern 1: Early return with ?
fn process_data() -> Result<(), AppError> {
    let data = fetch_data()?;            // Get data or return error
    let processed = process(data)?;       // Process or return error
    save_results(processed)?;             // Save or return error
    Ok(())                               // Everything worked!
}

// Pattern 2: Collecting multiple results
let results: Result<Vec<_>, _> = vec![1, 2, 3]
    .iter()
    .map(|&x| divide(10.0, x as f64))  // Try division for each number
    .collect();                        // Collect into Result

// Pattern 3: Handling multiple error types
fn complex_operation() -> Result<(), AppError> {
    // Each operation might fail with a different error type
    let file = File::open("data.txt").map_err(AppError::IoError)?;
    let num = "123".parse::<i32>().map_err(AppError::ParseError)?;
    Ok(())
}
```

## 6. Working with Option Types - Handling Missing Values

### What is Option?
`Option` is for when a value might not exist. It's like a container that's either:
- `Some(value)` - "Here's the value you wanted"
- `None` - "Sorry, no value here"

### Option Basics
```rust
// Option's basic structure
enum Option<T> {
    Some(T),  // Contains a value of type T
    None,     // No value
}

// Function that might not find what we're looking for
fn find_user(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))  // Found Alice!
    } else {
        None                         // User not found
    }
}
```

### Option Combinators - Working with Maybe-Values
```rust
// map: Transform the value if it exists
let user = find_user(1)
    .map(|name| format!("User: {}", name))     // Add "User: " prefix
    .unwrap_or_else(|| String::from("Unknown")); // Default if not found

// Chaining operations on Option
let user_length = find_user(1)        // Try to find user
    .map(|name| name.len())          // If found, get name length
    .unwrap_or(0);                   // If not found, use 0
```

## 7. Understanding unwrap() and Related Methods

The `unwrap()` method is a quick way to get the value inside a `Result` or `Option`, but it comes with risks:

```rust
// unwrap() on Result
let number: Result<i32, &str> = Ok(42);
let value = number.unwrap();  // Gets 42
// But be careful! This would panic:
// let error_case: Result<i32, &str> = Err("oops");
// error_case.unwrap();  // PANIC: "oops"

// unwrap() on Option
let some_value: Option<i32> = Some(42);
let value = some_value.unwrap();  // Gets 42
// But this would panic:
// let none_case: Option<i32> = None;
// none_case.unwrap();  // PANIC: "called `Option::unwrap()` on a `None` value"
```

Think of `unwrap()` like opening a package:
- If there's something inside (Ok/Some), you get the value
- If it's empty or damaged (Err/None), your program crashes

Here are safer alternatives to `unwrap()`:

```rust
// 1. expect() - Like unwrap() but with a custom error message
let file = File::open("config.txt")
    .expect("Config file should be present!");  // Better error message if it fails

// 2. unwrap_or() - Provide a default value if there's an error
let age: Option<i32> = None;
let default_age = age.unwrap_or(18);  // Use 18 if age is None

// 3. unwrap_or_else() - Provide a function for default value
let age: Option<i32> = None;
let calculated_default = age.unwrap_or_else(|| {
    // Complex calculation for default value
    20 + 5
});

// 4. unwrap_or_default() - Use the type's default value
let name: Option<String> = None;
let empty_name = name.unwrap_or_default();  // Gets empty String

// 5. match - Most verbose but most control
let number: Result<i32, &str> = Ok(42);
let value = match number {
    Ok(n) => n,
    Err(e) => {
        println!("Error: {}", e);
        -1  // Default value
    }
};
```

When to use each:
1. `unwrap()`: Only in examples or when you're 100% sure it won't fail
2. `expect()`: When you want to crash with a helpful message if something goes wrong
3. `unwrap_or()`: When you have a simple default value
4. `unwrap_or_else()`: When computing the default value is expensive
5. `unwrap_or_default()`: When the type's default value is appropriate
6. `match`: When you need complete control over both success and failure cases

Best Practices:
```rust
// ❌ Bad: Dangerous unwrap
fn get_user_data() -> String {
    let file = File::open("user.txt").unwrap();  // Might panic!
    // ...
}

// ✅ Good: Handle the error properly
fn get_user_data() -> Result<String, std::io::Error> {
    let file = File::open("user.txt")?;  // Propagate the error
    // ...
}

// ✅ Also Good: Provide meaningful defaults
fn get_config_value(key: &str) -> String {
    some_config_lookup(key)
        .unwrap_or_else(|| format!("default_{}", key))
}
```

Remember:
- `unwrap()` is like using scissors to open a package without checking if you might cut what's inside
- It's better to handle potential errors gracefully
- If you must use `unwrap()`, use `expect()` instead to provide context
- In production code, prefer error handling with `match`, `?`, or the safer alternatives shown above

## Key Takeaways
1. Use `Result` for operations that might fail
2. Use `Option` for values that might not exist
3. Use `panic!` only for unrecoverable errors
4. The ? operator makes error handling code cleaner
5. Custom error types help organize error handling
6. Good error messages help users understand what went wrong

Remember: Rust's error handling might seem complex at first, but it helps you write more reliable programs by making you think about what can go wrong and how to handle it.
