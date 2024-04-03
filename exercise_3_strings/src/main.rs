/**
 * Welcome to the ALTEN rust playground. In this exercise you will create some strings.
 * Did you know String allocates on the heap and str immutable stored _somewhere_?
 *
 * To run this exercise you can run tests via `cargo test` or run using `cargo run` in this directory.
 *
 */
fn main() {
    // Create a string with some wise words
    println!("Hello, {}", wise_saying());
    println!("Hello, {}", create_some_wise_words());


    // String welding
    let s1: String = "Hello".to_string();
    let s2: String = "Rust".to_string();
    let s3 = string_welder(s1, s2);
    println!("{}", s3);


    // Take a slice 🍕️
    let s: String = "Hello, Rust!".to_string();
    let slice: &str = slice_first_five(&s);
    println!("{}", slice);
}


// Return a str with "wise!"
fn wise_saying() -> &'static str {
    todo!("Return a str")
}

// Return a String with "SomeWords"
fn create_some_wise_words() -> String {
    todo!("Return a String")
}

// Return concatenation of 'a + " " + b'
fn string_welder(a: String, b: String) -> String {
    todo!("Return string a + b with a space in between")
}

// Return a str slice of the first 5 characters
fn slice_first_five<'a>(s: &'a String) -> &'a str {
    todo!("Return a slice of the first 5 characters")
}

#[cfg(test)]
mod tests {
    use crate::{wise_saying, create_some_wise_words, string_welder, slice_first_five};

    #[test]
    fn test_wise_saying() {
        let result = wise_saying();
        assert_eq!(result, "wise!");
    }

    #[test]
    fn test_create_some_wise_words() {
        let result = create_some_wise_words();
        assert_eq!(result, String::from("SomeWords"));
    }

    #[test]
    fn test_string_welder() {
        let result = string_welder("Hello".to_string(), "Rust".to_string());
        assert_eq!(result, String::from("Hello Rust"));
    }

    #[test]
    fn test_slice_first_five() {
        let s = "Hello this is a long sentence".to_string();
        let result = slice_first_five(&s);
        assert_eq!(result, String::from("Hello"));
    }
}