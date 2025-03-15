use std::collections::HashMap;

use actix::prelude::*;

use crate::{actors::shared::messages::SimpleMessage, utils::generate_code::generate_code};

use super::messages::{Connect, Relay};

pub struct SessionManager {
    // the key is the session code and the value will be (Session, Session) so the users
    pub sessions: HashMap<String, (Option<Recipient<SimpleMessage>>, Option<Recipient<SimpleMessage>>)>,
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
    type Result = String;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_code: String;
        // if session code already exists 
        if msg.session_code.is_some() {
            session_code = msg.session_code.clone().unwrap();
            println!("{:?}", msg.session_code);

            // inform the other guy that this guy has joined
            // Get a tuple (immutable borrow)
            if let Some((guy0, _)) = self.sessions.get(&session_code).as_deref() {
                if guy0.is_some() {
                    guy0.clone().unwrap().do_send(SimpleMessage("the other guy joined".to_string()));
                }
            }

             // Modify a tuple element (requires mutable access)
            if let Some(guys) = self.sessions.get_mut(&session_code) {
                guys.1 = Some(msg.sender.to_owned());
            }
        }
        // else we create a new session
        else {
            session_code = generate_code(7);

            self.sessions.insert(session_code.to_owned(), (Some(msg.sender.to_owned()), None));
        }

        msg.sender.do_send(SimpleMessage(session_code.to_owned()));

        session_code
    }
}


impl Handler<Relay> for SessionManager {
    type Result = ();

    fn handle(&mut self, msg: Relay, _ctx: &mut Self::Context) -> Self::Result {
        // Get a tuple (immutable borrow)
        if let Some((guy0, guy1)) = self.sessions.get(&msg.session_code).as_deref() {
            if guy0.is_some() && guy1.is_some() {
                let guy0_copy = guy0.clone().unwrap();
                let guy1_copy = guy1.clone().unwrap();

                if guy0_copy == msg.from {
                    // this means message is for guy1
                    guy1_copy.do_send(SimpleMessage(msg.msg));
                } else if guy1_copy == msg.from {
                    // this means message is for guy0
                    guy0_copy.do_send(SimpleMessage(msg.msg));
                }
            }
        }
    }
}