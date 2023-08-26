/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 */

 fn main() {
    // No need to change here, look at the functions to create a working implementation
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


/// Given two numbers the result is a combination of a + b
fn add_these_numbers(a: i32, b: i32) -> i32 {
    0
}


/// Returns the sum of given integer vector
///
/// # Arguments
///
/// * `v` - A vector containing integers
///
fn sumvec(...) -> i32 {
}


/// Divide given inputs where the result is a / b.
/// If b equals to 0.0 the error "Cannot divide by zero" is returned.
///
/// # Arguments
///
/// * `a` - The dividend of the operation
/// * `b` - The divisor of the operation
// fn divide(a: f64, b: f64) -> Result<f64, &'static str> {

// }


/// Divide function adapted with a String error type. This method maps all errors into one known type
/// to match across multiple interfaces in a function.
///
/// You can use .map_err() to convert the given error into a String
/// format!("") will help format strings like you did in println!().
///
/// # Arguments
///
/// * `a` - The dividend of the operation
/// * `b` - The divisor of the operation
// fn divide_custom_error(a: f64, b: f64) -> Result<f64, String> {
//     divide(a, b) // Add to this function call
// }

/// Optional assignment to work with enums.

/// Enum with 4 operations
/// 
/// # Values
/// 
/// * `Add` - Adds the given values
/// * `Sub` - Subtracts the given values
/// * `Mul` - Multiplies the given values
/// * `Div` - Divides the given values
enum Operation {
}

/// Generic calculate function requiring two inputs to create a result.
///
/// # Arguments
///
/// * `a` - Argument 1
/// * `b` - Argument 2
/// * `op` - Operation to execute on given arguments
fn calculate(a: f64, b: f64, op: Operation) -> f64 {
    0.0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add_these_numbers() {
        let result = crate::add_these_numbers(10, 30);
        assert_eq!(40, result);
    }

    #[test]
    fn test_sumvec() {
        let result = crate::sumvec(&[2, 3, 4]);
        assert_eq!(9, result);
    }

    // #[test]
    // fn test_divide() {
    //     let result = crate::divide(10.0, 2.0).unwrap();
    //     assert_eq!(5., result);
    // }

    // #[test]
    // fn test_divide_by_zero() {
    //     let result = crate::divide_custom_error(10., 0.).unwrap_err();
    //     assert_eq!("Division error! Cannot divide by zero", result);
    // }


    // Enable these tests for optional final part

    // #[test]
    // fn test_calculate_add() {
    //     let result = crate::calculate(2., 4., crate::Operation::Add);
    //     assert_eq!(6., result);
    // }

    // #[test]
    // fn test_calculate_sub() {
    //     let result = crate::calculate(2., 4., crate::Operation::Sub);
    //     assert_eq!(-2., result);
    // }

    // #[test]
    // fn test_calculate_mul() {
    //     let result = crate::calculate(2., 4., crate::Operation::Mul);
    //     assert_eq!(8., result);
    // }

    // #[test]
    // fn test_calculate_div() {
    //     let result = crate::calculate(2., 4., crate::Operation::Div);
    //     assert_eq!(0.5, result);
    // }
}
