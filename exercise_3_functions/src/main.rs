// /**
//  * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
//  */
// fn main() {
//     println!("Hello, world! I calculated 1 + 2 = {}", add_these_numbers(1,2));
// }

// fn add_these_numbers(a: i32, b: i32) -> i32 {
//     a + b
// }

// /// Returns the sum of given integer vector
// /// 
// /// # Arguments
// /// 
// /// * `v` - A vector containing integers
// /// 
// /// # Examples
// ///
// /// ```
// /// // You can have rust code between fences inside the comments
// /// // If you pass --test to `rustdoc`, it will even test it for you!
// /// let sum = sumvec([2,3,4]);
// /// ```
// fn sumvec(v: &[i32]) -> i32 {
//     v.iter().sum()
// }


// // INTERMEDIATE

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {

    if b == 0.0 {

        Err("Cannot divide by zero")

    } else {

        Ok(a / b)

    }

}

fn main() {

    match divide(10.0, 2.0) {
        Ok(result) => println!("The result is: {}", result),
        Err(err) => println!("Error: {}", err),

    }


    let result = divide(10, 2).map_err(|e| format!("Division error! {}", e)
}





// // MASTER
// enum Operation {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }

// fn calculate(a: f64, b: f64, op: Operation) -> f64 {
//     match op {
//         Operation::Add => a + b,
//         Operation::Sub => a - b,
//         Operation::Mul => a * b,
//         Operation::Div => a / b,
//     }
// }

// fn main() {
//     let result = calculate(10.0, 5.0, Operation::Div);
//     println!("The result is: {}", result);
// }