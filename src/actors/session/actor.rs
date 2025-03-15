use actix::prelude::*;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

use crate::actors::session_manager::actor::SessionManager;

pub struct Session {
    pub manager: Addr<SessionManager>,
    pub code: Option<String>,
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<Message, ProtocolError>> for Session {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            Message::Text(text) => {
                println!("{:?}", text);
                ctx.text(text);
            }
            Message::Binary(_) => println!("Unexpected binary"),
            Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            Message::Continuation(_) => {
                ctx.stop();
            }
            Message::Nop => (),
            Message::Ping(_) => {}
            Message::Pong(_) => {}
        };
    }
}

// impl Handler<SimpleMessage> for Session {
//     type Result = ();

//     fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {}
// }
