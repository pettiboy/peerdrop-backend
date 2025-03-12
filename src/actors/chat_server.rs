use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    u64,
};

use crate::messages::chat_server::{
    ChatMessage, ClientMessage, Connect, Disconnect, Join, ListRooms,
};
use actix::prelude::*;
use rand::Rng;

// manages chat rooms and responsible for coordinating chat session
pub struct ChatServer {
    // mapping of session_id to [ Address (recipient of a ChatMessage) ]
    // phonebook
    sessions: HashMap<u64, Recipient<ChatMessage>>,

    // mapping of name of chatroom
    // to the users (session_id) set currently in the chat room
    rooms: HashMap<String, HashSet<u64>>,

    // total number of connected users in the chat server
    // AtomicUsize: An atomic unsigned-sized integer. This allows the counter to be safely incremented and decremented by multiple threads concurrently without race conditions
    // Arc: An atomically reference-counted pointer. This allows the AtomicUsize to be safely shared between multiple threads (e.g., the main actor thread and other worker threads). The Arc ensures that the counter is only deallocated when all threads have finished using it
    visitor_count: Arc<AtomicUsize>,
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            visitor_count,
        }
    }
}

impl ChatServer {
    // send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: u64) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(ChatMessage(message.to_string()));
                    }
                }
            }
        }
    }
}

// Handler for Connect message
// Register new
impl Handler<Connect> for ChatServer {
    type Result = u64;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        self.send_message("main", "Someone joined", 0);

        // register session with random id
        let id = rand::rng().random::<u64>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        self.rooms
            // gets the `Entry` for the key == `main` in the `Hashmap`
            .entry("main".to_owned())
            // if `main` key already exists - it returns mut ref to exiting value
            // if not - inserts the default value (empty Hashset<u64>) and returns mut ref to it
            .or_default()
            // inserts the id to the Hashset<u64>
            .insert(id);

        // update count in a thread safe way
        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);

        // inform the room with updated visitor count
        self.send_message("main", &format!("Total visitors {count}"), 0);

        // send id back
        id
    }
}

// Handler for Disconnect Message
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        println!("Someone disconnected");

        // used to store the names of the rooms from which user disconnects
        let mut rooms: Vec<String> = Vec::new();

        // if a session with the session_id exists
        if self.sessions.remove(&msg.id).is_some() {
            // remove session (user) from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
    }
}

// Handler for ClientMessage Message
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.room, &msg.msg, msg.id);
    }
}

// Handler for ListRooms Message
impl Handler<ListRooms> for ChatServer {
    type Result = Vec<String>;

    fn handle(&mut self, _msg: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();

        for room_name in self.rooms.keys() {
            rooms.push(room_name.to_owned());
        }

        rooms
    }
}

// Join room, send disconnect message to old room
// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        // send disconnect message to old room
        let mut rooms = Vec::new();

        // remove session(user) from all rooms
        for (room_name, sessions) in self.rooms.iter_mut() {
            // Removes a value from the set.
            // Returns whether the value was present in the set.
            if sessions.remove(&msg.id) {
                rooms.push(room_name.to_owned());
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        // join room
        self.rooms
            .entry(msg.name.to_owned())
            .or_default()
            .insert(msg.id);

        // send join message to all
        self.send_message(&msg.name, "Someone connected", msg.id);
    }
}
