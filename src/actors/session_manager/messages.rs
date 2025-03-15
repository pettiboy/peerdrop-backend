use actix::prelude::*;

use crate::actors::shared::messages::SimpleMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub session_code: Option<String>, // if null then create a new entry in the sessions hashmap
    pub sender: Recipient<SimpleMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Relay {
    pub session_code: String,
    pub from: Recipient<SimpleMessage>,
    pub msg: String,
}
