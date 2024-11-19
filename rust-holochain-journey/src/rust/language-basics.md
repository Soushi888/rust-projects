# Language Basics

## Variables and Mutability

In Rust, variables are immutable by default. This means once a value is bound to a name, you can't change that value.

```rust
let x = 5; // immutable
// x = 6; // This would cause an error

let mut y = 5; // mutable
y = 6; // This is allowed
```

### Shadowing
You can declare a new variable with the same name as a previous variable:

```rust
let x = 5;
let x = x + 1; // x is now 6
let x = x * 2; // x is now 12
```

## Data Types

### Scalar Types

#### Integers
- Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

```rust
let a: i32 = -42;
let b: u32 = 42;
```

#### Floating-Point
- `f32`: Single precision
- `f64`: Double precision (default)

```rust
let x = 2.0; // f64 by default
let y: f32 = 3.0; // f32
```

#### Boolean
```rust
let t = true;
let f: bool = false;
```

#### Character
```rust
let c = 'z';
let heart_eyed_cat = '😻';
```

### Compound Types

#### Tuples
Fixed-length collection of values of different types:

```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup; // Destructuring
let first = tup.0; // Access by index
```

#### Arrays
Fixed-length collection of values of the same type:

```rust
let arr = [1, 2, 3, 4, 5];
let first = arr[0];
let arr_with_same_value = [3; 5]; // [3, 3, 3, 3, 3]
```

## Functions as I/O Systems

Functions in Rust can be thought of as input/output systems that process data. They can:
1. Accept inputs (parameters)
2. Process the data
3. Produce an output (return value)

### Basic Function Structure
```rust
//              inputs       output
//              ↓↓↓↓↓↓        ↓↓↓
fn process(x: i32, y: i32) -> i32 {
    x + y  // Processing and output
}
```

### Input Types
Functions can accept zero or more inputs as parameters:
```rust
// No input
fn say_hello() {
    println!("Hello!");
}

// Single input
fn square(x: i32) -> i32 {
    x * x
}

// Multiple inputs
fn combine_strings(s1: &str, s2: &str) -> String {
    format!("{} {}", s1, s2)
}
```

### Output Types
Functions can have different output behaviors:
```rust
// No output (returns unit type '()')
fn log_number(x: i32) {
    println!("Number is: {}", x);
}

// Single output
fn double(x: i32) -> i32 {
    x * 2
}

// Multiple outputs using tuple
fn split_string(s: &str) -> (String, String) {
    let mid = s.len() / 2;
    let first = s[..mid].to_string();
    let second = s[mid..].to_string();
    (first, second)
}
```

### Early Returns
Functions can have multiple exit points using `return`:
```rust
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        return None;  // Early return for invalid input
    }
    Some(x / y)      // Normal return path
}
```

## Control Flow

### If Expressions
```rust
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

#### loop
Infinite loop until explicitly broken:

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

#### while
Conditional loop:

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### for
Iterate over collections:

```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

for number in (1..4) {
    println!("{}!", number);
}
```

## Pattern Matching

### match Expression
```rust
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
use std::io; // Import a specific module
use std::io::Write; // Import a specific trait
use std::{io, fs}; // Import multiple items
use std::io::*; // Import all public items
```

### Module Creation
```rust
mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

use math::add;
```

## Memory Management

### Ownership
- Each value has an owner
- Only one owner at a time
- Value is dropped when owner goes out of scope

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}", s1); // This would cause an error
```

### References
Borrow values without taking ownership:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);
```

For practical examples of these concepts, see our [CLI Projects](../projects/cli/README.md).
