use actix::prelude::*;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

use crate::actors::{session_manager::{actor::SessionManager, messages::{Connect, Relay}}, shared::messages::SimpleMessage};

pub struct Session {
    pub manager: Addr<SessionManager>,
    pub code: Option<String>,
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // so when the Session actor is started - we inform the SessionManager
        // if new   - creates a code and sends it back
        // else     - relays existing code
        // this logic is handled inside the Connect handler

        // Addr<SessionManager> is actually a lightweight handle
        //  (similar to a reference) to the actor, not the actor itself...
        // when we call clone() on it, we're just creating another handle that 
        //  points to the same underlying SessionManager actor
        let manager = self.manager.to_owned();

        let code = self.code.to_owned();

        manager
        // the connect message send back a code
        // to this Session Actor
        .send(Connect {
            // for `.recipient()` to work here we have to write a handler for `SimpleMessage`
            sender: ctx.address() // gets THIS actor's address (Session actor in this case)
                        .recipient(),            // creates a way for OTHER actors to send messages TO this actor
            session_code: code
        })
        .into_actor(self)
        .then(|res, act, _ctx| {
            match res {
                Ok(code) => {
                    act.code = Some(code);
                }
                _ => println!("Something is wrong"),
            }
            fut::ready(())
        })
        .wait(ctx);
        
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Session {
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
                self.manager.do_send(Relay{
                    session_code: self.code.clone().unwrap_or("".to_string()),
                    from: ctx.address().recipient(),
                    msg: text.to_string()
                });
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
