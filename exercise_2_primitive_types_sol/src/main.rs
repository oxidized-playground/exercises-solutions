/**
 * Welcome to the ALTEN rust playground. In this exercise you will create lists and tuples using basic data types
 *
 */
fn main() {
    let x: bool = true;
    let p: &str = "🍕️";
    println!("Hello, world! We have {}? {}", p, x);

    // Create an array of 5 integers and print all of them
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    for item in list {
        println!("Have {}!", item);
    }
    println!("{:?}", list);

    // Create tuple of 2 values
    let tup = (34, 36);
    println!("{:?}", tup);
}
