use actix::prelude::*;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

use crate::actors::{session_manager::{actor::SessionManager, messages::Connect}, shared::messages::SimpleMessage};

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

                self.manager
                    // the connect message send back a code
                    // to this Session Actor
                    .do_send(Connect {
                        // for `.recipient()` to work here we have to write a handler for `SimpleMessage`
                        sender: ctx.address() // gets THIS actor's address (Session actor in this case)
                                    .recipient(),            // creates a way for OTHER actors to send messages TO this actor
                        session_code: self.code.to_owned()
                    })

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

// makes .recipient()
impl Handler<SimpleMessage> for Session {
    type Result = ();

    // so whenever another actor sends a SimpleMessage to this recipient
    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {
        // we just forward it as is
        ctx.text(msg.0);
    }
}
