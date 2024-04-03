/// Divide given inputs where the result is a / b.
/// If b equals to 0.0 the error "Cannot divide by zero" is returned.
///
/// # Arguments
///
/// * `a` - The dividend of the operation
/// * `b` - The divisor of the operation
pub fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    todo!("Implement a division that gives an error as mentioned above");
}

/// Divide function adapted with a String error type. This method maps all errors into one known type
/// to match across multiple interfaces in a function.
///
/// You can use map_err to convert the given error into a String
/// format!("") will help format strings like you did in println!().
///
/// # Arguments
///
/// * `a` - The dividend of the operation
/// * `b` - The divisor of the operation
pub fn divide_custom_error(a: f64, b: f64) -> Result<f64, String> {
    todo!("Call divide(a,b) and prefix the error with 'Division error! '");
}

#[cfg(test)]
mod tests {
    use crate::c_divide::{divide, divide_custom_error};

    #[test]
    fn test_divide() {
        let result = divide(10.0, 2.0).unwrap();
        assert_eq!(5., result);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide_custom_error(10., 0.).unwrap_err();
        assert_eq!("Division error! Cannot divide by zero", result);
    }
}
