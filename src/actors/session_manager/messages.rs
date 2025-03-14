use actix::prelude::*;

use crate::actors::shared::messages::SimpleMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    session_code: String,
    sender: Recipient<SimpleMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Relay {
    session_code: String,
    from: Recipient<SimpleMessage>,
    msg: String,
}
