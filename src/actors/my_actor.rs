// https://actix.rs/docs/actix/getting-started

use crate::messages::my_actor::Ping;
use actix::prelude::*;

pub struct MyActor {
    pub count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

// handlers for this actor

// we need to declare that our actor MyActor can accept Ping and handle it
impl Handler<Ping> for MyActor {
    type Result = usize;

    // all handlers have to have a handle function
    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}
