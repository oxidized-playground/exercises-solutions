/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 *
 * To run this exercise you can run tests via `cargo test` or run using `cargo run` in this directory.
 *
 */

pub mod part_1_add_numbers;
use part_1_add_numbers::add_these_numbers;
pub mod part_2_sumvec;
use part_2_sumvec::sumvec;
pub mod part_3_divide;
use part_3_divide::divide;
pub mod part_4_calculate;





fn main() {
    // part 1 Enable the functions if you want to print some results. Tests will validate your implementations
    println!(
        "Hello, world! I calculated 1 + 2 = {}",
        add_these_numbers(1, 2);
    );

    // Part 2, enable the following lines and fill in the function
    let sum = crate::sumvec(&vec![2, 3, 4]);
    println!("Sum of the vector is {}", sum);

    // Part 3, enable the following lines and fill in the function
    match divide(10.0, 2.0) {
        Ok(result) => println!("The division result is: {}", result),
        Err(err) => println!("Error: {}", err),
    };

    // Part 4 Optional assignment for enums, enable the following lines and last tests at bottom
    let result = calculate(10.0, 5.0, Operation::Div);
    println!("The calculation result is: {}", result);
}
