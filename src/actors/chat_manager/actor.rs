use actix::prelude::*;

use crate::actors::shared::{messages::SimpleMessage, types::MessageData};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_id: u64,
    pub sender_address: Recipient<SimpleMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Authenticate {
    pub session_id: u64,
    pub user_code: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct KeyExchange {
    pub data: MessageData,
}
