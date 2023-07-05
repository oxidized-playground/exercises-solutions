use serde::{Deserialize, Serialize};
use std::{str, time};

mod net;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

type ErrorMessage = std::string::String;

#[tokio::main]
async fn main() -> Result<(), ErrorMessage> {
    main_loop().await
}

async fn main_loop() -> Result<(), ErrorMessage> {
    let remote_address = "127.0.0.1:8080";
    let mut client = net::client::Client::new(remote_address).await?;

    let message = "{\"sender\":\"MyName\",\"content\":\"Hello!\"}";
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
