/// Given two numbers the result is a combination of a + b
pub fn add_these_numbers(a: i32, b: i32) -> i32 {
    0
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_add_these_numbers() {
        let result = crate::add_these_numbers(10, 30);
        assert_eq!(40, result);
    }
}
