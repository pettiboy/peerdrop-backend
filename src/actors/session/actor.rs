use actix::prelude::*;
use actix_web_actors::ws::{ProtocolError, WebsocketContext};

use crate::actors::{session_manager::actor::SessionManager, shared::messages::SimpleMessage};

pub struct Session {
    pub manager: Addr<SessionManager>,
    pub code: Option<String>,
}

impl Actor for Session {
    type Context = WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<actix_web_actors::ws::Message, ProtocolError>> for Session {
    fn handle(
        &mut self,
        item: Result<actix_web_actors::ws::Message, ProtocolError>,
        ctx: &mut Self::Context,
    ) {
    }
}

impl Handler<SimpleMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {}
}
