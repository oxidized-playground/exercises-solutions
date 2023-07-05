use serde::{Deserialize, Serialize};
use std::{io, str};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,
    content: String,
}

type ErrorMessage = std::string::String;

async fn process(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];

    println!("Started connection to {}", socket.peer_addr()?);

    loop {
        let amount_read = socket.read(&mut buf).await?;
        if amount_read == 0 {
            println!("Dropped connection to {}", socket.peer_addr()?);
            return Ok(())
        }

        let response_str = match response(&buf[..amount_read]) {
            Ok(response_str) => response_str,
            Err(e) => {
                println!("Error in building response {}", e);
                format!("Error in building response {}", e)
            }
        };

        socket.write_all(response_str.as_bytes()).await?;
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:8080".to_string();
    let listener = TcpListener::bind(addr.clone()).await?;
    println!("Listening on: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move { process(socket).await });
    }
}

fn response(message_buf: &[u8]) -> Result<String, ErrorMessage> {
    let message = decode_message(message_buf)?;

    println!("[{}]: {}", message.sender, message.content);

    let response_message = Message {
        sender: "Server".to_string(),
        content: format!("greetings from {}", message.sender),
    };

    serde_json::to_string(&response_message)
        .map_err(|e| format!("Could not encode response: {}", e))
}

fn decode_message(message_buf: &[u8]) -> Result<Message, ErrorMessage> {
    let message_str =
        str::from_utf8(message_buf).map_err(|e| format!("Message is not UTF8: {}", e))?;

    serde_json::from_str(&message_str).map_err(|e| format!("Error in decoding json: {}", e))
}