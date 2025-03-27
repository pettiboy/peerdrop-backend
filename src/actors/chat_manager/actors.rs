use actix::prelude::*;

use crate::actors::shared::messages::SimpleMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub user_code: String,
    pub sender_address: Recipient<SimpleMessage>,
}
