use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

type ErrorMessage = String;

pub struct Client {
    socket: TcpStream,
}

impl Client {
    pub async fn new(remote_address: &str) -> Result<Self, ErrorMessage> {
        let socket = TcpStream::connect(remote_address)
            .await
            .map_err(|e| format!("Could not connect to remote {}: {}", remote_address, e))?;

        Ok(Client { socket })
    }

    pub async fn send(&mut self, message: &str) -> Result<(), ErrorMessage> {
        println!("Sending message {} to server", message);
        self.socket
            .write_all(message.as_bytes())
            .await
            .map_err(|e| format!("Could not send message! {}", e))?;

        Ok(())
    }

    pub async fn read(&mut self) -> Result<String, ErrorMessage> {
        let mut buf = [0; 1024];
        let amount_read = self.read_bytes(&mut buf).await?;

        let message_buf = &buf[..amount_read];
        let message_str = String::from_utf8(Vec::from(message_buf))
            .map_err(|e| format!("Message is not UTF8: {}", e))?;

        Ok(message_str)
    }

    pub async fn read_bytes(&mut self, buf: &mut [u8]) -> Result<usize, ErrorMessage> {
        let amount_read = self
            .socket
            .read(buf)
            .await
            .map_err(|e| format!("Could not receive data {}", e))?;

        Ok(amount_read)
    }
}
