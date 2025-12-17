# Project 05: Mini JSON Parser

## 🎯 Goal

Represent a complex, nested data structure (like JSON) using a Recursive Enum. The goal is to understand how Rust handles data types that can contain themselves (Recursion) and how to traverse them.

## 💡 Key Concepts Learned

* **Recursive Enums:** An enum variant that holds a `Vec` of the enum itself (e.g., `Array(Vec<JsonValue>)`).
* **Complex Instantiation:** creating deeply nested objects manually in code.
* **Pattern Matching:** using `match` to distinguish between a simple Number and a complex Object.
* **The `Display` Trait:** (Optional Challenge) Customizing how the enum prints so it looks like real JSON strings, rather than Rust debug output.

> 📖 **Read More:** [Rust Book Ch 06: Match Control Flow](https://doc.rust-lang.org/book/ch06-02-match.html) | [Rust Book Ch 15: Box (Context for Recursion)](https://doc.rust-lang.org/book/ch15-01-box.html)

## 📝 The Assignment

1. Define an enum `JsonValue` with these variants:
   * `Null`
   * `Boolean(bool)`
   * `Number(f64)`
   * `String(String)`
   * `Array(Vec<JsonValue>)`
   * `Object(Vec<(String, JsonValue)>)`
2. In `main.rs`, construct a variable that represents this specific JSON data:

   ```json
        {
            "name": "Rust",
            "versions": [1.0, 2015, 2018, 2021],
            "is_awesome": true
        }
    ```

*(Hint: You will need to nest `JsonValue::Array` inside `JsonValue::Object`)*.
3. Print the result using `{:?}` (Debug format).

## 💻 Usage

Run the project from the root directory:

```bash
cargo run -p json_parser
```

### ⚠️ Setup Reminder

Remember the naming rule for the last time!

```bash
cargo new 05-json-parser --name json_parser
```

### Test Case 1: The Assignment Requirement

This matches the JSON structure asked for in the README.

**Target JSON:**

```json
{
  "name": "Rust",
  "versions": [1.0, 2015, 2018, 2021],
  "is_awesome": true
}
```

**Rust Construction (Copy into `main`):**

```rust
let data = JsonValue::Object(vec![
    // 1. String Field
    (
        String::from("name"), 
        JsonValue::String(String::from("Rust"))
    ),
    
    // 2. Array Field
    (
        String::from("versions"), 
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2015.0),
            JsonValue::Number(2018.0),
            JsonValue::Number(2021.0),
        ])
    ),

    // 3. Boolean Field
    (
        String::from("is_awesome"), 
        JsonValue::Boolean(true)
    )
]);

println!("{:#?}", data);
```

---

### Test Case 2: Advanced (Deep Nesting)

If you want to test if your code handles **Objects inside Objects** (Recursion), try this one.

**Target JSON:**

```json
{
  "user_id": 101,
  "profile": {
     "full_name": "Sanjay",
     "active": true
  }
}

```

**Rust Construction:**

```rust
let complex_data = JsonValue::Object(vec![
    // Simple Number
    (
        String::from("user_id"),
        JsonValue::Number(101.0)
    ),
    
    // Object inside Object (Recursion)
    (
        String::from("profile"),
        JsonValue::Object(vec![
            (
                String::from("full_name"),
                JsonValue::String(String::from("Sanjay"))
            ),
            (
                String::from("active"),
                JsonValue::Boolean(true)
            )
        ])
    )
]);

println!("{:#?}", complex_data);

```

**Tip:** Notice the usage of `.to_string()` or `String::from()`? You need those because the keys in a Hash/Map are Strings, not string slices (`&str`).
