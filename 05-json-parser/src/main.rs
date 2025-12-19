#[derive(Debug)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
fn main() {
    let complex_data = JsonValue::Object(vec![
        // Simple Number
        (String::from("user_id"), JsonValue::Number(101.0)),
        // Object inside Object (Recursion)
        (
            String::from("profile"),
            JsonValue::Object(vec![
                (
                    String::from("full_name"),
                    JsonValue::String(String::from("Sanjay")),
                ),
                (String::from("active"), JsonValue::Boolean(true)),
            ]),
        ),
    ]);

    // {:#?} (the "pretty-print" formatter),
    println!("{:#?}", complex_data);
}
