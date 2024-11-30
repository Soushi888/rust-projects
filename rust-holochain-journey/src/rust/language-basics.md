# Rust Language Basics

## Introduction
Welcome to Rust! If you're new to programming or coming from another programming language, you're in the right place. Rust is a modern programming language that focuses on three key principles:
1. **Safety**: Rust prevents common programming mistakes at compile-time
2. **Concurrency**: Rust makes it easier to write programs that use multiple processors efficiently
3. **Performance**: Rust code runs as fast as C and C++

This guide will take you through the basics, assuming no prior Rust knowledge.

## 1. Variables and Types

### What is a Variable?
A variable is like a labeled box that holds a value. Think of it as giving a name to a piece of data so you can use it later.

### Variables and Mutability
In Rust, variables are "immutable" (cannot be changed) by default. This helps prevent accidental changes to your data.

```rust
// Immutable variable - like writing in stone
let x = 5;          // Once set to 5, x can never change
// x = 6;          // ❌ This would cause an error!

// Mutable variable - like writing in sand
let mut y = 5;      // Adding 'mut' makes y changeable
y = 6;              // ✅ This is allowed because y is mutable
```

### Basic Data Types
Rust needs to know exactly what type of data you're working with. Here are the most common types:

```rust
// Numbers
let integer: i32 = 42;        // Whole numbers (i32 means 32-bit integer)
let float: f64 = 3.14;        // Decimal numbers (f64 means 64-bit float)
let boolean: bool = true;     // true or false values
let character: char = 'A';    // Single characters (notice the single quotes)

// Compound Types
let tuple: (i32, f64) = (42, 3.14);     // A group of different types
let array: [i32; 3] = [1, 2, 3];        // A list of same-type items

// Accessing tuple values
let first_number = tuple.0;    // Gets 42 (tuples start counting at 0)
let pi = tuple.1;             // Gets 3.14

// Accessing array values
let first_item = array[0];    // Gets 1 (arrays start counting at 0)
```

### String Types
Rust has two main ways to handle text:
1. `&str` (string slice) - Like a view into some text
2. `String` - Like a box that owns and can modify text

```rust
// String literal (&str) - Think of it as borrowed text
let greeting = "Hello";       // Fixed text that can't change

// Owned String - Think of it as your own notepad
let mut name = String::from("World");    // Text you can change
name.push_str("!");                      // Adds "!" to the end

// Combining strings
let message = format!("{} {}", greeting, name);  // "Hello World!"
```

## 2. Functions and Control Flow

### What is a Function?
A function is like a recipe - it takes ingredients (parameters), follows steps, and produces a result. Functions help organize code into reusable pieces.

### Function Basics
```rust
// Basic function that adds two numbers
fn add(x: i32, y: i32) -> i32 {    // -> i32 means "returns a number"
    x + y  // No semicolon means "return this value"
}

// Function that might not have a result
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None                // Return nothing if dividing by zero
    } else {
        Some(x / y)        // Return the result if safe
    }
}

// Using our functions
let sum = add(5, 3);           // sum becomes 8
let result = divide(10.0, 2.0); // result becomes Some(5.0)
let bad = divide(10.0, 0.0);    // bad becomes None
```

### Control Flow
Control flow is how your program makes decisions and repeats tasks.

```rust
// If expressions - Making decisions
let number = 6;
if number % 2 == 0 {           // % 2 checks if number is even
    println!("It's even!");
} else {
    println!("It's odd!");
}

// Match expressions - Like a sophisticated switch statement
let favorite_color = "blue";
match favorite_color {
    "red" => println!("Red like roses!"),
    "blue" => println!("Blue like the sky!"),
    "green" => println!("Green like grass!"),
    _ => println!("Some other color!"),  // _ means "anything else" or "default"
}
```

### Loops - Doing Things Repeatedly
```rust
// For loop - When you know how many times to repeat
for i in 0..5 {              // Counts from 0 to 4
    println!("Count: {}", i);
}

// While loop - Repeat while a condition is true
let mut count = 0;
while count < 5 {
    println!("Count: {}", count);
    count += 1;              // Add 1 to count
}

// Loop - Keep going until told to stop
let mut count = 0;
let result = loop {          // Keep going until break
    count += 1;
    if count == 10 {
        break count;         // Stop and return count
    }
};
```

## 3. Structs and Methods

### What is a Struct?
A struct is like a custom blueprint for creating data. It lets you group related pieces of data together.

### Basic Struct
```rust
// Define a blueprint for a Person
struct Person {
    name: String,           // Every person has a name
    age: u32,              // and an age (u32 means positive number)
}

// Add functionality to Person
impl Person {
    // Constructor - like a factory for making new People
    fn new(name: String, age: u32) -> Person {
        Person { name, age }    // Create a new Person
    }

    // Method - something a Person can do
    fn introduce(&self) {       // &self means "the Person using this method"
        println!("Hi! I'm {} and I'm {} years old", self.name, self.age);
    }
}

// Using our Person struct
let alice = Person::new(String::from("Alice"), 30);
alice.introduce();  // Prints: Hi! I'm Alice and I'm 30 years old
```

## 4. Enums - Creating Custom Types with Multiple Variants

### What is an Enum?
An enum (enumeration) is a type that can be one of several variants. Think of it like a set of options where a value must be exactly one of those options. For example, a card suit must be either Hearts, Diamonds, Clubs, or Spades - nothing else.

### Basic Enum
```rust
// Define an enum for card suits
enum CardSuit {
    Hearts,    // Each variant is like a possible value
    Diamonds,
    Clubs,
    Spades,
}

// Using the enum
let my_suit = CardSuit::Hearts;  // Use :: to access a variant

// Match with enums
match my_suit {
    CardSuit::Hearts => println!("♥"),
    CardSuit::Diamonds => println!("♦"),
    CardSuit::Clubs => println!("♣"),
    CardSuit::Spades => println!("♠"),
}
```

### Enums with Data
Enum variants can hold data, making them very powerful:

```rust
// Enum where each variant can hold different types of data
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields like a struct
    Write(String),             // Single piece of data
    ChangeColor(i32, i32, i32), // Multiple pieces of data
}

// Using enum with data
let messages = vec![
    Message::Quit,
    Message::Move { x: 3, y: 4 },
    Message::Write(String::from("hello")),
    Message::ChangeColor(255, 0, 0),
];

for msg in messages {
    match msg {
        Message::Quit => println!("Quit application"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({},{},{})", r, g, b),
    }
}
```

### Methods on Enums
Just like structs, enums can have methods:

```rust
enum Shape {
    Circle(f64),                // radius
    Rectangle(f64, f64),        // width, height
    Triangle(f64, f64, f64),    // three sides
}

impl Shape {
    // Calculate area for any shape variant
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// Using the enum method
let shapes = vec![
    Shape::Circle(5.0),
    Shape::Rectangle(4.0, 3.0),
    Shape::Triangle(3.0, 4.0, 5.0),
];

for shape in shapes {
    println!("Area: {:.2}", shape.area()); // Prints area to 2 decimal places
}
```

### Option Enum - A Special Case
Rust has a built-in enum called `Option` that's used everywhere:

```rust
// Option is defined like this in the standard library
enum Option<T> {
    Some(T),   // Contains a value of type T
    None,      // Represents no value
}

// Using Option
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None                // Return None for division by zero
    } else {
        Some(numerator / denominator)  // Return Some containing the result
    }
}

// Working with Option values
let result = divide(10.0, 2.0);
match result {
    Some(x) => println!("Result: {}", x),
    None => println!("Cannot divide by zero"),
}

// Shorter way using if let
if let Some(x) = divide(10.0, 2.0) {
    println!("Result: {}", x);
}
```

### Result Enum - Another Special Case
`Result` is another built-in enum for handling success and failure:

```rust
// Result is defined like this in the standard library
enum Result<T, E> {
    Ok(T),    // Success case containing value of type T
    Err(E),   // Error case containing error of type E
}

// Using Result
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(String::from("Failed to parse number")),
    }
}

// Working with Results
match parse_number("42") {
    Ok(num) => println!("Number: {}", num),
    Err(e) => println!("Error: {}", e),
}

// Using if let with Result
if let Ok(num) = parse_number("42") {
    println!("Got number: {}", num);
}
```

### Pattern Matching with Enums
Rust ensures you handle all possible enum variants:

```rust
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

fn get_activity(weather: Weather) -> String {
    match weather {
        Weather::Sunny => String::from("Go for a walk!"),
        Weather::Cloudy => String::from("Could be good for photography"),
        Weather::Rainy => String::from("Read a book inside"),
        Weather::Snowy => String::from("Build a snowman"),
        // No need for _ => ... because we covered all cases!
    }
}

// If we only care about some cases
let weather = Weather::Sunny;
if let Weather::Sunny = weather {
    println!("It's a beautiful day!");
}
```

Remember: Enums are one of Rust's most powerful features because they:
1. Make impossible states impossible to represent
2. Keep related data and logic together
3. Work great with pattern matching
4. Help catch errors at compile time

## 5. Collections

### What are Collections?
Collections are ways to store multiple values together. They're like containers for your data.

### Vectors (Dynamic Arrays)
A vector is like a growable list - it can hold any number of items of the same type.

```rust
// Creating vectors
let mut numbers: Vec<i32> = Vec::new();  // Empty vector
numbers.push(1);                         // Add 1
numbers.push(2);                         // Add 2

// Vector with initial values
let numbers = vec![1, 2, 3];            // Shorthand for creating with values

// Getting values
let first = &numbers[0];                // Get first item (might crash)
let first = numbers.get(0);             // Safer way to get first item

// Looking at all items
for number in &numbers {
    println!("Got number: {}", number);
}
```

## 6. Traits and Generics

### What are Traits?
Traits are like contracts that types can fulfill. They define shared behavior between different types.

### Defining Traits
```rust
// Define a contract for what makes something an Animal
trait Animal {
    // Every Animal must implement this
    fn make_sound(&self) -> String;
    
    // Animals get this for free
    fn description(&self) -> String {
        String::from("Just an animal")
    }
}

// Make a Cat type that follows the Animal contract
struct Cat {
    name: String,
}

// Implement the Animal contract for Cat
impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")           // Cats say meow
    }
    
    fn description(&self) -> String {
        format!("A cat named {}", self.name)
    }
}

// Using our Animal
let whiskers = Cat { name: String::from("Whiskers") };
println!("Sound: {}", whiskers.make_sound());      // Prints: Meow!
println!("Who: {}", whiskers.description());       // Prints: A cat named Whiskers
```

## 7. Memory Management

### Understanding Ownership
Rust's most unique feature is its ownership system. Think of it like lending books:
1. Each value has one owner (like a book has one owner)
2. When the owner goes away, the value is cleaned up (like donating a book when moving)
3. You can lend out values (like lending a book), but there are rules

```rust
// Ownership example
let s1 = String::from("hello");    // s1 owns the string
let s2 = s1;                       // Ownership moves to s2
// println!("{}", s1);             // ❌ Error: s1 no longer owns anything

// Cloning (making a copy)
let s3 = String::from("hello");
let s4 = s3.clone();              // Make a copy instead of moving
println!("{}", s3);               // ✅ OK: s3 still owns its copy
```

### References and Borrowing
References are like lending - you can use something without owning it.

```rust
// Borrowing example
fn calculate_length(s: &String) -> usize {  // Borrow s temporarily
    s.len()                                 // Return the length
}

let s = String::from("hello");
let len = calculate_length(&s);    // Lend s to the function
println!("{} is {} long", s, len); // s is still ours to use
```

## Next Steps
Now that you understand the basics, try:
1. Writing small programs to practice these concepts
2. Reading the error handling guide
3. Exploring the standard library documentation
4. Working on the practical projects in this guide

Remember: Rust's compiler is your friend! When you get errors, read them carefully - they usually tell you exactly how to fix the problem.