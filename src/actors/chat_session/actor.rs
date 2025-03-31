use actix::prelude::*;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use serde_json;

use crate::actors::{
    chat_manager::{
        actor::{Authenticate, Connect},
        message::ChatManager,
    },
    shared::{
        messages::SimpleMessage,
        types::{MessageType, WebSocketMessage},
    },
};

pub struct ChatSession {
    pub manager: Addr<ChatManager>,
    pub code: Option<String>,
    pub session_id: u64,
}

impl Actor for ChatSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.do_send(Connect {
            session_id: self.session_id,
            sender_address: ctx.address().recipient(),
        });
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for ChatSession {
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
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(ws_message) => match ws_message.data.message {
                        MessageType::Connect => {
                            println!("hello");
                            self.manager.do_send(Authenticate {
                                session_id: self.session_id,
                                user_code: "hello".to_owned(),
                            });
                        }
                        _ => {}
                    },
                    Err(e) => {
                        println!("invalid message {:?}", e);
                        ctx.address()
                            .recipient()
                            .do_send(SimpleMessage("invalid_message_sent".to_owned()));
                    }
                }
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
impl Handler<SimpleMessage> for ChatSession {
    type Result = ();

    // so whenever another actor sends a SimpleMessage to this recipient
    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {
        // we just forward it as is
        ctx.text(msg.0);
    }
}
