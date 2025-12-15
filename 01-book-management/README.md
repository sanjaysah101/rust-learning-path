# Project 01: Library Management System

## 🎯 Goal

Build a system to track books and their borrow status. The goal is to understand how data is structured in Rust and how `impl` blocks work.

## 💡 Key Concepts Learned

* **Structs:** Defining custom data types (`Book`).
* **Enums:** Handling state (`BookStatus::Available`, `CheckedOut`).
* **Impl Blocks:** Attaching logic/methods to data.
* **Ownership:** Why we need `&mut self` to change data.

> 📖 **Read More:** [Rust Book Ch 5: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html) | [Rust Book Ch 6: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)

## 📝 The Assignment

Create a `Book` struct that can change its status between `Available`, `CheckedOut`, and `Maintenance`. The system should prevent checking out a book that is already gone.

## 💻 Usage

Run the project from the root directory:

```bash
cargo run -p book_management
```
