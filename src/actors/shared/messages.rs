use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SimpleMessage(pub String);
