use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{str, time};

mod net;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

#[async_trait]
trait ALTENChatter {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage>;
    async fn read_message(&mut self) -> Result<Message, ErrorMessage>;
}

#[async_trait]
impl ALTENChatter for net::client::Client {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage> {
        // todo!();
        let message = serde_json::to_string(&message).unwrap();
        self.send(&message).await
    }

    async fn read_message(&mut self) -> Result<Message, ErrorMessage> {
        // todo!()
        let mut buf = [0; 1024];
        let amount_read = self.read(&mut buf).await?;

        let message_buf = &buf[..amount_read];
        let message_str =
            str::from_utf8(message_buf).map_err(|e| format!("Message is not UTF8: {}", e))?;

        serde_json::from_str(&message_str).map_err(|e| format!("Error in decoding json: {}", e))
    }
}

type ErrorMessage = std::string::String;

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "localhost:8080";
    let mut client = net::client::Client::new(remote_address).await?;

    let msg = Message {
        sender: String::from("Koen"),
        content: String::from("AAAA"),
    };

    loop {
        client.write_message(&msg).await?;
        let reply = client.read_message().await?;
        println!("Received: {:?}", reply);

        let ten_seconds = time::Duration::from_millis(10);
        std::thread::sleep(ten_seconds);
    }
}
