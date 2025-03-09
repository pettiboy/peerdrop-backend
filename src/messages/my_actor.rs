use actix::prelude::*;

// define the Message that the actor needs to accept
#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

// any actor that can accept a Ping message needs to return usize value.
