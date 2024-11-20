# Language Basics

## Variables and Mutability

In programming, a variable is a name given to a value. In Rust, variables are immutable by default. This means once a value is bound to a name, you can't change that value. Variables are useful for storing and reusing values in your code, and they are also useful for making your code more readable.

```rust
// Variables are immutable by default in Rust
let x = 5; // This variable cannot be modified
// x = 6; // This would cause an error because x is immutable

let mut y = 5; // Using 'mut' keyword makes the variable mutable
y = 6; // This is allowed because y is mutable
```

### Shadowing
You can declare a new variable with the same name as a previous variable:

```rust
// Shadowing allows us to reuse variable names
let x = 5;          // First x
let x = x + 1;      // Create new x, using the value of previous x
let x = x * 2;      // Create another new x, using the value of previous x
```

## Data Types

### Scalar Types

#### Integers
- Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

```rust
// Signed integers can hold both positive and negative values
let a: i32 = -42;   // 32-bit signed integer
// Unsigned integers can only hold positive values
let b: u32 = 42;    // 32-bit unsigned integer
```

#### Floating-Point
- `f32`: Single precision
- `f64`: Double precision (default)

```rust
// Floating point numbers for decimal values
let x = 2.0;        // f64 type (double precision) is the default
let y: f32 = 3.0;   // Explicitly specified as f32 (single precision)
```

#### Boolean
```rust
// Boolean type can only be true or false
let t = true;       // Type inference determines this is a bool
let f: bool = false; // Explicitly typed as bool
```

#### Character
```rust
// Char type represents a Unicode scalar value
let c = 'z';            // Single ASCII character
let heart_eyed_cat = '😻'; // Unicode emoji character
```

### Compound Types

#### Tuples
Fixed-length collection of values of different types:

```rust
// Tuples can store multiple values of different types
let tup: (i32, f64, bool) = (500, 6.4, true);  // Explicit type annotation
let (x, y, z) = tup;    // Destructuring: assign each value to a variable
let first = tup.0;      // Access tuple elements using dot notation and index
```

#### Arrays
Fixed-length collection of values of the same type:

```rust
// Arrays are fixed-length collections of same-type elements
let arr = [1, 2, 3, 4, 5];          // Array initialization
let first = arr[0];                  // Array indexing (zero-based)
let arr_with_same_value = [3; 5];    // Initialize array with 5 elements, all set to 3
```

## Functions as I/O Systems

Functions in Rust can be thought of as input/output systems that process data. They can:
1. Accept inputs (parameters)
2. Process the data
3. Produce an output (return value)

### Basic Function Structure
```rust
// Function that takes two i32 parameters and returns an i32
//             inputs        output
//             ↓↓↓↓↓↓         ↓↓↓
fn process(x: i32, y: i32) -> i32 {
    x + y  // Implicit return (no semicolon needed)
}
```

### Input Types
Functions can accept zero or more inputs as parameters:
```rust
// Function with no parameters and no return value
fn say_hello() {
    println!("Hello!");
}

// Function that takes one parameter and returns its square
fn square(x: i32) -> i32 {
    x * x
}

// Function that combines two string slices into a new String
fn combine_strings(s1: &str, s2: &str) -> String {
    format!("{} {}", s1, s2)
}
```

### Output Types
Functions can have different output behaviors:
```rust
// Function that only prints - returns unit type '()'
fn log_number(x: i32) {
    println!("Number is: {}", x);
}

// Function that doubles its input and returns the result
fn double(x: i32) -> i32 {
    x * 2
}

// Function returning multiple values using a tuple
fn split_string(s: &str) -> (String, String) {
    let mid = s.len() / 2;
    let first = s[..mid].to_string();    // First half of string
    let second = s[mid..].to_string();   // Second half of string
    (first, second)
}
```

### Early Returns
Functions can have multiple exit points using `return`:
```rust
// Function demonstrating early return with Option type
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;  // Early return for division by zero
    }
    Some(x / y)      // Return Some containing the division result
}
```

## Control Flow

Control flow refers to the order in which instructions in a program are executed. It's the heart of programming, as it allows us to make decisions, repeat actions, and skip over certain parts of code. Control flow is important because it allows us to write more efficient and flexible code. It also makes it easier to debug our code when something doesn't work as intended.


### If Expressions



```rust
// Example of if-else control flow
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4 or 3");
}
```

### Loops

Loops are a fundamental control flow construct used to execute a set of instructions multiple times. Loops are important because they allow us to write more efficient and less repetitive code. They also make it easier to debug our code when something doesn't work as intended. Loops are also used to iterate over collections of data, such as arrays or vectors, and to perform operations on each element in the collection.


#### loop
Infinite loop until explicitly broken:

```rust
// Loop with a break condition returning a value
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;    // Break with a value to be assigned to result
    }
};
```

#### while
Conditional loop:

```rust
// While loop with a countdown
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### for
Iterate over collections:

```rust
// For loop iterating over array elements
let a = [10, 20, 30, 40, 50];
for element in a.iter() {     // Iterate over array elements
    println!("the value is: {}", element);
}

// For loop with range
for number in (1..4) {        // Iterate over range 1 to 3 (4 is exclusive)
    println!("{}!", number);
}
```

## Pattern Matching

### match Expression
```rust
// Example of pattern matching with match expression
let number = 13;
match number {
    1 => println!("One!"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
}
```

## Error Handling

For detailed error handling examples and explanations, see [Error Handling](./error-handling.md).

## Imports and Modules

### Basic Import
```rust
// Import a specific module
use std::io;
// Import a specific trait
use std::io::Write;
// Import multiple items
use std::{io, fs};
// Import all public items
use std::io::*;
```

### Module Creation
```rust
// Create a new module named 'math'
mod math {
    // Public function within the module
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

// Use the 'add' function from the 'math' module
use math::add;
```

## Memory Management

### Ownership
- Each value has an owner
- Only one owner at a time
- Value is dropped when owner goes out of scope

```rust
// Ownership example
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}", s1); // This would cause an error
```

### References
Borrow values without taking ownership:

```rust
// Function that takes a reference to a String and returns its length
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
```

For practical examples of these concepts, see our [CLI Projects](../projects/cli/README.md).
