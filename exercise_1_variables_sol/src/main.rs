/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 *
 * To run this exercise you can either use the build in tools of VSCode or start via `cargo run` in this directory.
 *
 */
fn main() {
    // Introduce another variable here to make the print statement work

    // Mutable variable x of type int 32
    let mut x: i32 = 10;
    let y = 3.4;
    println!("Hello, world! X: {}, Y: {}", x, y);

    // Modify x here, give it a new value. Rust offers two methods to do this, can you find them both?
    x = 12;
    println!("Hello, world! X: {}, Y: {}", x, y);
    let x = 13;
    println!("Hello, world! X: {}, Y: {}", x, y);

    // Swap x and y here in one action
    let (x, y) = (y, x);
    println!("Hello, world! X: {}, Y: {}", x, y);
}
