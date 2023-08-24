fn main() {
    // Create a string with some wise words
    println!("Hello, {}", wiseSaying());
    println!("Hello, {}", createSomeWiseWords());


    // String welding
    let s1: String = "Hello".to_string();
    let s2: String = "Rust".to_string();
    let s3 = s1 + " " + &s2;
    println!("{}", s3);


    // Take a slice 🍕️
    let s: String = "Hello, Rust!".to_string();
    let slice: &str = &s[0..5];
    println!("{}", slice);
}

fn wiseSaying() -> &'static str {
    "aa"
}

fn createSomeWiseWords() -> String {
    String::from("bb")
}

// Test to push_str() 

