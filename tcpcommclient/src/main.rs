use serde::{Deserialize, Serialize};
use std::{str, time};

mod net;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

// trait ChatChecker {
//     fn check_sum(self) -> bool;
// }

// impl ChatChecker for net::client::Client {
//     fn check_sum(self) -> bool {
//         self.read().contains("ALTEN")
//     }
// }

// trait ALTENChatter {
//     fn write(&mut self, message: &String);
//     fn read(self) -> String;
// }

// impl ALTENChatter for net::client::Client {
//     fn write(&mut self, message: &String) {
//         // todo!();
//         self.send(message);
//     }

//     fn read(self) -> String {
//         todo!()
//     }
// }

type ErrorMessage = std::string::String;

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "localhost:8080";
    let mut client = net::client::Client::new(remote_address).await?;

    let msg = Message{sender: String::from("K"), content: String::from("AAAA")};
    let message = serde_json::to_string(&msg).unwrap();

    // let message = "{\"sender\":\"Koen\",\"content\":\"Hello!\"}";
    

    let mut buf = [0; 1024];
    loop {
        client.send(&message).await?;
        
        let amount_read = client.read(&mut buf).await?;

        let message_buf = &buf[..amount_read];
        let message_str =
            str::from_utf8(message_buf).map_err(|e| format!("Message is not UTF8: {}", e))?;
        println!("Received: {:?}", message_str);

        let ten_seconds = time::Duration::from_millis(10);
        std::thread::sleep(ten_seconds);
    }
}
