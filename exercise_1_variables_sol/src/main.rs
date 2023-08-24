/**
 * Welcome to the ALTEN rust playground. In this exercise you will create a new variable and print its content.
 */
fn main() {
    // Use cargo run to build and start the exercise

    let mut x = 10;
    let y = 3.4;
    println!("Hello, world! X: {}, Y: {}",  x, y);

    // Modify x here
    x = 12;
    println!("Hello, world! X: {}, Y: {}",  x, y);

    // Swap x and y here
    let (x,y) = (y,x);
    println!("Hello, world! X: {}, Y: {}",  x, y);
}
