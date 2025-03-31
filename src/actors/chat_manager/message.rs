use std::collections::HashMap;

use actix::prelude::*;

use crate::actors::shared::{
    messages::SimpleMessage,
    types::{MessageData, MessageType},
};

use super::actor::{Authenticate, Connect};

pub struct ChatManager {
    // key is User.code
    pub connected_users: HashMap<String, Recipient<SimpleMessage>>,
    // key is the ChatSession.session_id
    pub pending_users: HashMap<u64, Recipient<SimpleMessage>>,
}

impl Actor for ChatManager {
    type Context = Context<Self>;
}

impl ChatManager {
    pub fn new() -> ChatManager {
        ChatManager {
            connected_users: HashMap::new(),
            pending_users: HashMap::new(),
        }
    }
}

impl Handler<Connect> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        // check valid user and signature (later)

        // push msg sender in connected users
        self.pending_users
            .insert(msg.session_id.to_owned(), msg.sender_address.to_owned());

        // let response = MessageData {
        //     sender: "server".to_string(),
        //     recipient: Some(msg.user_code.to_owned()),
        //     message: MessageType::ConnectAck,
        // };

        // let response_string = serde_json::to_string(&response).unwrap();

        msg.sender_address
            .do_send(SimpleMessage("send_authenticate".to_string()));
    }
}

impl Handler<Authenticate> for ChatManager {
    type Result = ();

    fn handle(&mut self, msg: Authenticate, _ctx: &mut Self::Context) -> Self::Result {
        // get recipient from session id mapping
        // and clone it so we can keep it in scope after removing this reference
        let recipient = self
            .pending_users
            .get(&msg.session_id)
            .expect("invalid session id")
            .to_owned();

        // push msg sender in connected users
        self.connected_users
            .insert(msg.user_code.to_owned(), recipient.to_owned());

        // remove the session id from pending users
        self.pending_users
            .remove(&msg.session_id)
            .expect("unable to remove session id from pending users");

        let response = MessageData {
            sender: "server".to_string(),
            recipient: Some(msg.user_code.to_owned()),
            message: MessageType::ConnectAck,
        };

        let response_string = serde_json::to_string(&response).unwrap();

        recipient.do_send(SimpleMessage(response_string));
    }
}
