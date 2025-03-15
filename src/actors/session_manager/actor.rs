use std::collections::HashMap;

use actix::prelude::*;

use crate::actors::shared::messages::SimpleMessage;

use super::messages::Connect;

pub struct SessionManager {
    pub code: Option<String>,
    pub sessions: HashMap<String, (Recipient<SimpleMessage>, Recipient<SimpleMessage>)>,
}

impl Actor for SessionManager {
    type Context = Context<Self>;
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager {
            code: None,
            sessions: HashMap::new(),
        }
    }
}

impl Handler<Connect> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {}
}
