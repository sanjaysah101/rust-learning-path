enum BookStatus {
    Available,
    CheckedOut(String),
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    status: BookStatus,
}

impl Book {
    // Associative function
    fn new(title: String, author: String, pages: u32) -> Self {
        Self {
            title,
            author,
            pages,
            status: BookStatus::Available,
        }
    }

    fn borrow_book(&mut self, user_name: String) -> Result<bool, String> {
        let is_available = match &self.status {
            BookStatus::Available => true,
            _ => false,
        };

        match is_available {
            true => {
                self.status = BookStatus::CheckedOut(user_name.clone());
                println!("{} have successfully borrowed the book.", user_name);
                Ok(true)
            }
            false => Err(String::from("Sorry, the book is already checked out.")),
        }
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
        println!("You have successfully returned the book.");
    }
}

fn main() {
    let mut rust_book = Book::new(String::from("Rust Book"), String::from("Sanjay Sah"), 32);

    let result = rust_book.borrow_book(String::from("Sanjay Sah"));

    match result {
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("{}", e),
    }

    let result = rust_book.borrow_book(String::from("Sanjay Kumar"));

    match result {
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("{}", e),
    }
    // let _ = rust_book.borrow_book(String::from("user_name"));
    rust_book.return_book();
}
