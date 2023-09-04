/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 * Start with part A and work up to D. Part D is optional if you run out of time.
 */

 pub mod a_add_numbers;
 pub mod b_sumvec;
 pub mod c_divide;
 pub mod d_calculate;
 
 use a_add_numbers::add_these_numbers;
//  use b_sumvec::sumvec;
//  use c_divide::divide;

 
 fn main() {
    // Enable the functions if you want to print some results. Tests will validate your implementations
    println!(
        "Hello, world! I calculated 1 + 2 = {}",
        add_these_numbers(1, 2)
    );

    // Part 2, enable the following lines and fill in the function
    // let sum = crate::sumvec(...);
    // println!("Sum of the vector is {}", sum);

    // Part 3, enable the following lines and fill in the function
    // match divide(10.0, 2.0) {
    //     Ok(result) => println!("The division result is: {}", result),
    //     Err(err) => println!("Error: {}", err),
    // };


    // Optional assignment for enums, enable the following lines and last tests at bottom
    // let result = calculate(10.0, 5.0, Operation::Div);
    // println!("The calculation result is: {}", result);
}
