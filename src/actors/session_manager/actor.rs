use std::collections::HashMap;

use actix::prelude::*;

use crate::{actors::shared::messages::SimpleMessage, utils::generate_code::generate_code};

use super::messages::Connect;

pub struct SessionManager {
    // the key is the session code and the value will be (Session, Session) so the user
    pub sessions: HashMap<String, (Recipient<SimpleMessage>, Recipient<SimpleMessage>)>,
}

impl Actor for SessionManager {
    type Context = Context<Self>;
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager {
            sessions: HashMap::new(),
        }
    }
}

impl Handler<Connect> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_code: String;
        // if session code already exists 
        if msg.session_code.is_some() {
            session_code = msg.session_code.clone().unwrap();
            println!("{:?}", msg.session_code)
        }
        // else we create a new session
        else {
            session_code = generate_code(7);

        }
        msg.sender.do_send(SimpleMessage(session_code));
    }
}
