fn main() {
    enum Message {
        Temperature(f32), 
        Humidity(i8),
        MessageOfDay(String),
        Ping,
    }

    let message = Message::MessageOfDay("Welcome to Rust!".to_string());

    match message {
        Message::Temperature(temperature) => println!("The temperature is {}", temperature),
        Message::Humidity(humidity) => println!("The humidity is {}", humidity),
        Message::MessageOfDay(s) => println!("Message of the day: {}", s),
        Message::Ping => println!("Pong!"),
    }
}

fn functiefrunctie() -> Result<String, std::io::Error> {todo!()}

fn give_me_a_option() -> Option<String> {todo!()}