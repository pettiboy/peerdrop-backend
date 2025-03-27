use std::collections::HashMap;

use actix::prelude::*;

use crate::actors::shared::{
    messages::SimpleMessage,
    types::{MessageData, MessageType},
};

use super::actor::Connect;

pub struct ChatManager {
    // key is User.code
    pub connected_users: HashMap<String, Recipient<SimpleMessage>>,
}

impl Actor for ChatManager {
    type Context = Context<Self>;
}

impl ChatManager {
    pub fn new() -> ChatManager {
        ChatManager {
            connected_users: HashMap::new(),
        }
    }
}

impl Handler<Connect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        // check valid user and signature (later)

        // push msg sender in connected users
        self.connected_users
            .insert(msg.user_code.to_owned(), msg.sender_address.to_owned());

        let response = MessageData {
            sender: "server".to_string(),
            recipient: Some(msg.user_code.to_owned()),
            message: MessageType::ConnectAck,
        };

        let response_string = serde_json::to_string(&response).unwrap();

        msg.sender_address.do_send(SimpleMessage(response_string));
    }
}
