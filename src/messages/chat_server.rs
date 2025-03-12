use actix::prelude::*;

// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

// Message for chat server communications

// New chat session is created
#[derive(Message)]
#[rtype(u64)]
pub struct Connect {
    pub addr: Recipient<ChatMessage>
}

// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u64
}

// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    // Id of the client session (user_id)
    pub id: u64,
    // message
    pub msg: String,
    // room name
    pub room: String,
}

// List of available rooms
pub struct ListRooms;
impl Message for ListRooms {
    type Result = Vec<String>;
}

// Join room, if room does not exist creates a new one
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    // client id
    pub id: u64,

    // room name
    pub name: String,
}


