use actix::prelude::*;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use serde_json;
use sqlx::PgPool;

use crate::{
    actors::{
        chat_manager::{
            actor::{Authenticate, Connect, KeyExchange},
            message::ChatManager,
        },
        shared::{
            messages::SimpleMessage,
            types::{MessageType, ResponseMessages, WebSocketMessage},
        },
    },
    db,
    utils::eddsa::eddsa_verify_signature,
};

pub struct ChatSession {
    pub manager: Addr<ChatManager>,
    pub code: Option<String>,
    pub session_id: u64,
    pub db_pool: PgPool,
}

impl ChatSession {
    fn handle_authenticate(&self, ws_message: WebSocketMessage, ctx: &mut WebsocketContext<Self>) {
        let manager = self.manager.clone();
        let pool = self.db_pool.clone();
        let session_id = self.session_id;
        let addr = ctx.address();

        let user_code_claimed = ws_message.data.sender.clone();
        let given_signature = ws_message.signature;
        let given_data = serde_jcs::to_string(&ws_message.data).expect("invalid message given");
        println!("{:?}", given_data);

        // using actors context to handle the future (db operation)
        async move {
            match db::user::get_user(&pool, &user_code_claimed).await {
                Ok(user) => {
                    let public_key_from_db = &user.eddsa_public_key;

                    let is_valid_signature =
                        eddsa_verify_signature(&given_data, &given_signature, public_key_from_db);

                    if !is_valid_signature {
                        addr.do_send(SimpleMessage(
                            serde_jcs::to_string(&ResponseMessages::InvalidSignature).unwrap(),
                        ));
                    } else {
                        manager.do_send(Authenticate {
                            session_id,
                            user_code: user.code,
                        });
                    }
                }
                Err(_) => {
                    addr.do_send(SimpleMessage(
                        serde_json::to_string(&ResponseMessages::InvalidSender).unwrap(),
                    ));
                }
            }
        }
        .into_actor(self)
        .wait(ctx);
    }

    fn handle_key_exchange(&self, ws_message: WebSocketMessage, _ctx: &mut WebsocketContext<Self>) {
        self.manager.do_send(KeyExchange {
            data: ws_message.data,
        });
    }
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
                            println!("connecting..");
                            self.handle_authenticate(ws_message, ctx)
                        }
                        MessageType::KeyExchange { .. } => {
                            self.handle_key_exchange(ws_message, ctx)
                        }
                        _ => {}
                    },
                    Err(_) => {
                        ctx.address().recipient().do_send(SimpleMessage(
                            serde_json::to_string(&ResponseMessages::InvalidMessage).unwrap(),
                        ));
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
