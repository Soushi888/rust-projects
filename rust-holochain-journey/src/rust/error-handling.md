# Error Handling

Error handling in Rust is a critical aspect that ensures your programs can gracefully handle unexpected situations. Rust provides two primary types for managing errors: `Result` and `Option`.

## Result

The `Result` type is used for operations that can succeed or fail. It is an enum with two variants:
- `Ok(T)`: Indicates success and contains a value of type `T`.
- `Err(E)`: Indicates failure and contains an error value of type `E`.

Here's a basic example of using `Result`:

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}
```

In this example, `divide` returns a `Result` type that either contains the division result or an error message.

### Pattern Matching with Result

You can use pattern matching to handle `Result` values:

```rust
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
}
```

This allows you to execute different code paths depending on whether the operation succeeded or failed.

## Option

The `Option` type is used for values that may or may not be present. It is an enum with two variants:
- `Some(T)`: Contains a value of type `T`.
- `None`: Represents the absence of a value.

Here's an example of using `Option`:

```rust
fn find_index(vec: Vec<i32>, target: i32) -> Option<usize> {
    for (index, value) in vec.iter().enumerate() {
        if *value == target {
            return Some(index);
        }
    }
    None
}
```

In this example, `find_index` returns an `Option` that contains the index of the target value or `None` if the target is not found.

### Pattern Matching with Option

Pattern matching can also be used with `Option`:

```rust
match find_index(vec![1, 2, 3], 2) {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}
```

## Using `?` Operator

The `?` operator is a shorthand for propagating errors. It can be used with functions that return `Result` or `Option`. Here's how it works:

```rust
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

In this function, `?` is used to return the error immediately if `File::open` or `file.read_to_string` fails.

### Requirements for Using `?`

- **Same Error Type**: The error type of the `Result` being propagated must match the error type of the function using `?`. If they differ, you may need to convert the error type using `map_err` or another conversion method.

- **Return Type**: The function using `?` must return a `Result` or `Option`. The `?` operator cannot be used in functions that return other types.

### Advantages of Using `?`

- **Simplicity**: Reduces boilerplate code for error handling.
- **Readability**: Makes the code easier to read and understand.
- **Consistency**: Provides a uniform way to handle errors across different functions.

Understanding `Result` and `Option` is essential for writing robust Rust programs. For practical examples, see the [Basic File Reader CLI](../projects/cli/1-basic-file-reader.md).
