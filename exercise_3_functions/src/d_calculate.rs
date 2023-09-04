/// Optional assignment to work with enums. Enable the test below the functions!

/// Enum with 4 operations
/// 
/// # Values
/// 
/// * `Add` - Adds the given values
/// * `Sub` - Subtracts the given values
/// * `Mul` - Multiplies the given values
/// * `Div` - Divides the given values
pub enum Operation {
}

/// Generic calculate function requiring two inputs to create a result.
///
/// # Arguments
///
/// * `a` - Argument 1
/// * `b` - Argument 2
/// * `op` - Operation to execute on given arguments
pub fn calculate(a: f64, b: f64, op: Operation) -> f64 {
    0.0
}

// #[cfg(test)]
// mod tests {
//     use crate::d_calculate::{calculate, Operation};

//     #[test]
//     fn test_calculate_add() {
//         let result = calculate(2., 4., Operation::Add);
//         assert_eq!(6., result);
//     }

//     #[test]
//     fn test_calculate_sub() {
//         let result = calculate(2., 4., Operation::Sub);
//         assert_eq!(-2., result);
//     }

//     #[test]
//     fn test_calculate_mul() {
//         let result = calculate(2., 4., Operation::Mul);
//         assert_eq!(8., result);
//     }

//     #[test]
//     fn test_calculate_div() {
//         let result = calculate(2., 4., Operation::Div);
//         assert_eq!(0.5, result);
//     }
// }
