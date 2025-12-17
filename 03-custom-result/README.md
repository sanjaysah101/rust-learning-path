# Project 03: Generic Result Wrapper

## 🎯 Goal

Build a simplified version of Rust's built-in `Result` enum. The goal is to understand how **Generics** allow us to write code that can handle data of *any* type, and how Rust handles errors without exceptions.

## 💡 Key Concepts Learned

* **Generics (`<T, E>`):** defining types that are specified later by the user.
* **Enums with Data:** Storing different types of values inside enum variants (`Success(T)` vs `Failure(E)`).
* **Pattern Matching:** extracting values out of the enum safely.
* **Trait Bounds:** Restricting generics (e.g., `where T: Debug`) to ensure they can be printed.

> 📖 **Read More:** [Rust Book Ch 10: Generics](https://doc.rust-lang.org/book/ch10-00-generics.html) | [Rust Book Ch 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

## 📝 The Assignment

1. Define a generic enum `MyResult<T, E>` with two variants:
   * `Success(T)`: Contains the successful value.
   * `Failure(E)`: Contains the error information.
2. Implement a method `unwrap_or(self, default: T) -> T`:
   * If the result is `Success`, return the value inside.
   * If the result is `Failure`, return the `default` value provided.
3. Implement a helper method `print(&self)`:
   * Use a `match` statement to print whether the operation was a success or failure.
   * *Hint:* You will need to add `#[derive(Debug)]` or usage of the `Debug` trait for `T` and `E`.
4. Main Function:
    * Test it with `MyResult<i32, String>` (e.g., Success(10)) and `MyResult<i32, String>` (e.g., Failure("Something went wrong".to_string())).

## 💻 Usage

Run the project from the root directory:

```bash
cargo run -p custom_result
```
