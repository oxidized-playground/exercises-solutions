use crate::{net, ErrorMessage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

#[async_trait]
pub trait ALTENChatter {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage>;
    async fn read_message(&mut self) -> Result<Message, ErrorMessage>;
}

#[async_trait]
impl ALTENChatter for net::client::Client {
    async fn write_message(&mut self, message: &Message) -> Result<(), ErrorMessage> {
        let message = serde_json::to_string(&message).unwrap();
        self.send(&message).await
    }

    async fn read_message(&mut self) -> Result<Message, ErrorMessage> {
        let message_str = self.read().await?;
        serde_json::from_str(&message_str).map_err(|e| format!("Error in decoding json: {}", e))
    }
}
