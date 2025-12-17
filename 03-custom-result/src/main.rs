use std::fmt::Debug; // Import the Debug trait

// #[derive(Debug)]
enum MyResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> MyResult<T, E> {
    fn unwrap_or(self, default: T) -> T {
        match self {
            MyResult::Success(value) => value,
            MyResult::Failure(_error) => default,
        }
    }

    fn print(&self)
    where
        T: Debug,
        E: Debug,
    {
        match self {
            MyResult::Success(val) => println!("Success: {:?}", val),
            MyResult::Failure(err) => println!("Error: {:?}", err),
        }
    }
}

fn main() {
    let good_result: MyResult<i32, String> = MyResult::Success(100);

    good_result.print();
    // We use unwrap_or. Since it's Success, it should return 100.
    let value = good_result.unwrap_or(0);
    println!("Value is: {}", value);

    println!("---");

    // Scenario 2: A Failed Result (Integer, String)
    // Note: We specify <i32, String> so it matches the type of good_result logic
    let bad_result: MyResult<i32, String> =
        MyResult::Failure(String::from("Database connection lost"));

    bad_result.print();
    // We use unwrap_or. Since it's Failure, it should return the default (0).
    let value = bad_result.unwrap_or(0);
    println!("Value is: {}", value);
}
