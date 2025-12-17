# Project 04: Generic Inventory System

## 🎯 Goal

Build a flexible inventory system that can manage different categories of items (like Potions or Weapons) using **Trait Bounds**. The goal is to understand how to write generic structs that are "constrained" to only accept types that implement specific behaviors.

## 💡 Key Concepts Learned

* **Trait Bounds:** Writing `<T: Item>` to enforce that a generic type must implement specific methods.
* **Generic Structs:** Creating a struct `Inventory<T>` that wraps a Vector of generic items.
* **Code Reusability:** Writing the `total_value` logic once, and having it work automatically for Potions, Weapons, or any future item type.

> 📖 **Read More:** [Rust Book Ch 10: Trait Bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)

## 📝 The Assignment

1. Define a trait `Item` with two methods:
   * `fn name(&self) -> &str;`
   * `fn price(&self) -> f64;`
2. Create two structs, `Potion` and `Weapon`, and implement the `Item` trait for both.
3. Create a generic struct `Inventory<T>`:
   * It should hold a list of items: `items: Vec<T>`.
   * **Important:** You generally don't enforce the trait bound on the struct definition itself, but on the `impl` block.
4. Implement methods for `Inventory<T>` where `T: Item`:
   * `new() -> Self`
   * `add(&mut self, item: T)`
   * `total_value(&self) -> f64` (Sums the price of all items).
5. In `main`, create one inventory specifically for `Potion`s and calculate its value.

## 💻 Usage

Run the project from the root directory:

```bash
cargo run -p inventory_system
```
