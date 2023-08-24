use serde::{Deserialize, Serialize};
use serde_json::Serialize;
use std::{str, time};

mod net;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

trait ChatChecker {
    fn check_sum(self) -> bool;
}

impl ChatChecker for net::client::Client {
    fn check_sum(self) -> bool {
        self.read().contains("ALTEN")
    }
}

trait ALTENChatter {
    fn write(self, message: &str);
    fn read(self) -> String;
}

impl ALTENChatter for net::client::Client {
    fn write(self, message: &str) {
        todo!()
        self.send(message);
    }

    fn read(self) -> String {
        todo!()
    }
}

type ErrorMessage = std::string::String;

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "mx.kb7.nl:8080";
    let mut client = net::client::Client::new(remote_address).await?;

    let msg = Message(sender: "K", content: "AAAA");
let message = serde_json::t(msg);

    // let message = "{\"sender\":\"Koen\",\"content\":\"Hello!\"}";
    client.send(message).await?;

    let mut buf = [0; 1024];
    loop {
        let amount_read = client.read(&mut buf).await?;

        let message_buf = &buf[..amount_read];
        let message_str =
            str::from_utf8(message_buf).map_err(|e| format!("Message is not UTF8: {}", e))?;
        println!("Received: {:?}", message_str);

        let ten_seconds = time::Duration::from_secs(10);
        std::thread::sleep(ten_seconds);
    }
}
