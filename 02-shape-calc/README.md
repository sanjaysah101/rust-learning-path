# Project 02: Shape Area Calculator

## 🎯 Goal

Build a system that can calculate the area of different shapes (Circle, Rectangle, Triangle) using a unified interface. The goal is to understand how **Traits** allow different structs to share common behavior (Polymorphism).

## 💡 Key Concepts Learned

* **Traits:** Defining a shared behavior (`Measurable`) that multiple structs can implement.

* **Polymorphism:** Using `Box<dyn Trait>` to store different types (e.g., Circle and Rectangle) in the same Vector.

* **Vector Iteration:** Looping through a list of trait objects to call their methods dynamically.

* **Encapsulation:** Keeping the calculation logic inside the struct, not the main function.

> 📖 **Read More:** [Rust Book Ch 10: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) | [Rust Book Ch 17: Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

## 📝 The Assignment

1. Define a trait `Measurable`:

    * It should have a method `area(&self) -> f64`.
    * It should have a method `fn name(&self) -> &str;`.

2. Create three structs: `Circle`, `Rectangle`, and `Triangle`.

    * Circle (fields: `radius`).

    * Rectangle (fields: `width`, `height`).

    * Triangle (fields: `base`, `height`).

3. Implement `Measurable` for all three structs.
4. Main Function:

    * Create a `Vec<Box<dyn Measurable>>`. (This introduces you to Trait Objects, necessary for storing different types in one list).

    * Push instances of a circle, rectangle, and triangle into the vector.

    * Iterate through the vector and print the area of each shape.

## 💻 Usage

Run the project from the root directory using the package name:

```bash
cargo run -p shape_calc
```
