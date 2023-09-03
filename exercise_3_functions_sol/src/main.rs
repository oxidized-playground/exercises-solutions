/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 */

pub mod a_add_numbers;
pub mod b_sumvec;
pub mod c_divide;
pub mod d_calculate;

use a_add_numbers::add_these_numbers;
use b_sumvec::sumvec;
use c_divide::divide;

fn main() {
    // No need to change here, look at the functions to create a working implementation
    println!(
        "Hello, world! I calculated 1 + 2 = {}",
        add_these_numbers(1, 2)
    );

    let sum = crate::sumvec(&vec![2, 3, 4]);
    println!("Sum of the vector is {}", sum);

    match divide(10.0, 2.0) {
        Ok(result) => println!("The division result is: {}", result),
        Err(err) => println!("Error: {}", err),
    };
}
