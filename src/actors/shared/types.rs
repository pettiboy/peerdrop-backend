use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub data: MessageData,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    pub message: MessageType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum MessageType {
    Connect,
    ConnectAck,
    KeyExchange { ecdh_public_key: String },
    KeyExchangeAck { ecdh_public_key: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseMessages {
    SendAuthenticate,
    InvalidMessage,
    InvalidSignature,
    InvalidSender,
}
