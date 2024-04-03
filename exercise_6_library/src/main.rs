/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a struct and manage a library using arrays.
 * The content of this exercise is based on the Library exercise from https://github.com/google/comprehensive-rust/blob/0e4df4b50536a1b300d4618ea13390869aeb4ba7/src/exercises/day-2/book-library.rs
 *
 * Uncomment the code in the main function further down the file.
 *
 * To run this exercise you can run tests via `cargo test` or run using `cargo run` in this directory.
 *
 */

/// Library managing a vector of Books
///
struct Library {
    books: Vec<Book>,
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.title == other.title
    }
}

impl Library {
    fn new() -> Library {
        todo!("Make a new library")
    }

    fn len(&self) -> usize {
        todo!("Return the length of `self.books`")
    }

    fn is_empty(&self) -> bool {
        todo!("Return `true` if `self.books` is empty")
    }

    fn add_book(&mut self, book: Book) {
        todo!("Add a new book to `self.books`")
    }

    fn print_books(&self) {
        todo!("Iterate over `self.books` and each book's title and year")
    }

    fn oldest_book(&self) -> Option<&Book> {
        todo!("Return a reference to the oldest book (if any)")
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let library = Library::new();

    // println!("Our library is empty: {}", library.is_empty());

    // library.add_book(Book::new("Lord of the Rings", 1954));
    // library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    // println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());

    // library.print_books();

    // match library.oldest_book() {
    //    Some(book) => println!("My oldest book is {book}"),
    //    None => println!("My library is empty!"),
    // }

    // println!("Our library has {} books", library.len());
    // library.print_books();
}


// Make all these tests pass
#[cfg(test)]
mod tests {
    use crate::Book;
    use crate::Library;

    #[test]
    fn test_library_len() {
        let mut library = Library::new();
        assert_eq!(library.len(), 0);

        library.add_book(Book::new("Bob", 1000));
        library.add_book(Book::new("Ada", 1100));
        assert_eq!(library.len(), 2);
    }

    #[test]
    fn test_library_empty() {
        let mut library = Library::new();
        assert!(library.is_empty());

        library.add_book(Book::new("Bob", 1000));
        assert!(!library.is_empty());
    }

    #[test]
    fn test_book() {
        let bob = Book::new("Bob", 32);
        assert_eq!(bob.year, 32);
    }

    #[test]
    fn test_library() {
        let bob = Book::new("Bob", 32);
        let geek = Book::new("Geek", 100);
        let boot = Book::new("Boot", 400);
        let sub = Book::new("Sub", 50);

        let mut library = Library::new();
        library.add_book(bob.clone());
        library.add_book(geek);
        library.add_book(boot);
        library.add_book(sub);

        assert_eq!(library.len(), 4);
        assert_eq!(library.oldest_book().map(|b| b.title.as_str()), Some("Bob"));
    }
}
