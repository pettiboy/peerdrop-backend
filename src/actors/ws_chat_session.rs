use std::time::Instant;

use actix::prelude::*;
use actix_web_actors::ws;

use super::my_actor::MyActor;

pub struct WsChatSession {
    // unique session id
    pub id: u64,

    // client must send ping at least once per 10 secs (CLIENT_TIMEOUT)
    pub hb: Instant,

    // joined room
    pub room: String,

    // peer name
    pub name: Option<String>,

    // chat server
    pub addr: Addr<MyActor>
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    // Method is ca
}


// impl WsChatSession {
//     // helper method that sends ping to client every 5 secs (HEARTBEAT_INTERVAL)
//     // 
//     // also checks heartbeat from client
//     fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
//         // ctx.run
//     }
// }


