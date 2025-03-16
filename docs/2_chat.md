# Simple Chat

Now that we can build a simple echo server, lets build a simple chat app

```sequence
sequenceDiagram
    participant C1 as Client 1 Browser
    participant WS1 as Session 1
    participant SM as SessionManager
    participant WS2 as Session 2
    participant C2 as Client 2 Browser

    %% First client connection
    C1->>+WS1: WS Connect ws://localhost:9081/ws
    Note over WS1: new Session::new(manager, None)
    WS1->>+SM: Connect { session_code: None, addr }
    Note over SM: generate_code()<br/>sessions.insert(code, (Some(addr), None))
    SM-->>-WS1: session_code "ABC123"
    WS1->>C1: Text("Your session: ABC123")

    %% Second client connection
    C2->>+WS2: WS Connect ws://localhost:9081/ws?session=ABC123
    Note over WS2: new Session::new(manager, Some("ABC123"))
    WS2->>+SM: Connect { session_code: Some("ABC123"), addr }
    Note over SM: sessions.get_mut("ABC123")<br/>second = Some(addr)
    SM->>WS1: Message("Peer connected!")
    SM-->>-WS2: session_code "ABC123"
    WS2->>C2: Text("Connected to session!")

    %% Message exchange
    C1->>WS1: WS Message: Text("Hello!")
    Note over WS1: handle(ws::Message::Text)
    WS1->>+SM: Relay { msg: "Hello!", session: "ABC123" }
    Note over SM: find_peer() and forward
    SM->>-WS2: Message("Hello!")
    WS2->>C2: Text("Hello!")
```

## Session actor

From the echo implementation change `EchoSession` -> `Session`

The `Session` actor will now represent a `user`'s session

<details><summary>Boilerplate main.rs</summary>

```rust
use actix::prelude::*;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self, Message, ProtocolError, WebsocketContext};

// TODO manager actor

//  session actor
pub struct Session;

impl Actor for Session {
    type Context = WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<Message, ProtocolError>> for Session {
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
                ctx.text(text);
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

#[get("/ws/chat")]
async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let ws_actor = Session {};

    ws::start(ws_actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(chat_route))
        .bind(("127.0.0.1", 9082))?
        .run()
        .await
}
```

</details>

## Overview

1. `SessionManager`: Manages all active sessions
2. Individual `Session` actors: Represent connected users

## Messages and Handlers

Lets start with defining the Message Types

- So each `Actor` can `implement` a `Message`
- A `Handler` for a `Message` is a way to define behaviour for an `Actor` - if this struct has come then what to do
- `Messages` are also used for inter Actor communication

### SimpleMessage

This is shared between by both `actors`

It is used when the `SessionManager` wants to send a message to the end user

- it does so by sending this struct to the `Session` actor it has
- the `Session` actor then sends it to the user

```rust
// ---- Message Types
// -- Shared
#[derive(Message)]          // we derive from `Message` in `actix`
#[rtype(result = "()")]     // the handler of this `Message` doesnt return anything
pub struct SimpleMessage(pub String);
```

### Implement a `Handler` of `Message` for an `Actor`

Now that we have defined a `SimpleMessage` lets define what `Session` actor should do when it receives it

```rs
// handler for Session actor
impl Handler<SimpleMessage> for Session {
    type Result = ();

    // so whenever another actor sends a SimpleMessage to this recipient
    fn handle(&mut self, msg: SimpleMessage, ctx: &mut Self::Context) -> Self::Result {
        // we just forward it as is to the user whos session this is
        ctx.text(msg.0);
    }
}
```

## SessionManager Actor

Now that we have created a `Message` for our `Session` actor, lets go a step back and create our `SessionManager` actor

The `SessionManager` actor maintains a map of all active sessions and handles session creation and joining:

```rust
pub struct SessionManager {
    // Map of session code to tuple of (first user, second user)
    pub sessions: HashMap<String, (Option<Recipient<SimpleMessage>>, Option<Recipient<SimpleMessage>>)>,
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager {
            sessions: HashMap::new(),
        }
    }
}
```

### Connect Message

```rust
pub struct Connect {
    pub session_code: Option<String>,
    pub sender: Recipient<SimpleMessage>,
}
```

### Handling Connect Messages

When a client wants to connect, the `SessionManager` handles it in two ways:

1. **Creating a New Session**:

```rust
// when no session_code is provided
let session_code = generate_code(7);
self.sessions.insert(session_code.to_owned(), (Some(msg.sender.to_owned()), None));
```

2. **Joining Existing Session**:

```rust
// when session_code is provided
if let Some((guy0, _)) = self.sessions.get(&session_code).as_deref() {
    if guy0.is_some() {
        // Notify first user that someone joined
        guy0.clone().unwrap().do_send(SimpleMessage("the other guy joined".to_string()));
    }
}

// add second user to session
if let Some(guys) = self.sessions.get_mut(&session_code) {
    guys.1 = Some(msg.sender.to_owned());
}
```

<details><summary> <h3> Handle `Connect` for `SessionManager` </h3> </summary>

```
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
```

</details>
