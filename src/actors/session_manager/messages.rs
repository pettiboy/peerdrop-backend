use actix::prelude::*;

use crate::actors::shared::messages::SimpleMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_code: String,
    pub sender: Recipient<SimpleMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Relay {
    pub session_code: String,
    pub from: Recipient<SimpleMessage>,
    pub msg: String,
}
