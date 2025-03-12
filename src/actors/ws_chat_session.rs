use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws::{self, Message, ProtocolError};

use crate::messages::chat_server::{
    ChatMessage, ClientMessage, Connect, Disconnect, Join, ListRooms,
};

use super::chat_server::ChatServer;

// how ofter heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
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
    pub addr: Addr<ChatServer>,
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // checks client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting..");

                // notify chat server
                act.addr.do_send(Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // dont try to send a ping
                return;
            }

            // otherwise send a ping
            ctx.ping(b"")
        });
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    // Method is called on actor start
    fn started(&mut self, ctx: &mut Self::Context) {
        // start heartbeat process on session start
        self.hb(ctx);

        // register self in chat server
        let addr = ctx.address();

        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,

                    // something is wrong with chat server
                    Err(_) => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

// Handle messages from chat server, we simply send it to peer websocket
impl Handler<ChatMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

// WebSocket message handler
impl StreamHandler<Result<Message, ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Message::Pong(_) => {
                self.hb = Instant::now();
            }
            Message::Text(text) => {
                let m = text.trim();

                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, " ").collect();

                    match v[0] {
                        "/list" => {
                            // send ListRooms message to chat server and wait for
                            // response
                            println!("List rooms");
                            self.addr
                                .send(ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                            // .wait(ctx) stops all events in context
                            // so actor wont receive any new messages until it gets list of rooms back
                        }
                        "/join" => {
                            if v.len() == 2 {
                                v[1].clone_into(&mut self.room);
                                self.addr.do_send(Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });

                                ctx.text("joined");
                            } else {
                                ctx.text("Room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("name is required");
                            }
                        }

                        _ => ctx.text("unknown command"),
                    }
                } else {
                    // ref: Bind by reference during pattern matching ??
                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };

                    // send message to chat server
                    self.addr.do_send(ClientMessage {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
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
        }
    }
}
