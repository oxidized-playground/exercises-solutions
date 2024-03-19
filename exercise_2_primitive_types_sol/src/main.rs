/**
 * Welcome to the ALTEN rust playground. In this exercise you will create lists and tuples using basic data types
 *
 * To run this exercise you can either use the build in tools of VSCode or start via `cargo run` in this directory.
 *
 */
fn main() {
    let x: bool = true;
    let p: &str = "🍕️";
    println!("Hello, world! We have {}? {}", p, x);

    // Create an array of 5 i32's and print all of them
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    for item in list {
        println!("Have {}!", item);
    }

    // You can also print complete objects using the debug formatter :?
    println!("{:?}", list);

    // Create tuple of 2 values
    let tup = (34, 36);
    println!("{:?}", tup);
}
