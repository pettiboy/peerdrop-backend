# Simple Echo using Actix

## Setup

```zsh
cargo new echo
cd echo
```

```zsh
cargo add actix
cargo add actix_web_actors
```

## Working

### create an actor for every connected user

```rust
use actix::prelude::*;
use actix_web_actors::ws::WebsocketContext;

pub struct EchoSession;

impl Actor for EchoSession {
    type Context = WebsocketContext<Self>;
}
```

### lets create a get route for this

This a get route that is upgraded to a websocket connection

- We initialise the actor here
- it will complain that the `StreamHandler` is not implemented for for `EchoSession`

```rust
use actix_web::{get, web, Error, HttpRequest, HttpResponse};

#[get("/ws/echo")]
async fn echo_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let ws_actor = EchoSession {};

    ws::start(ws_actor, &req, stream)
}
```

### implement StreamHandler for EchoSession

```rust
use actix::prelude::*;
use actix_web_actors::ws::{self, Message, ProtocolError, WebsocketContext};

impl actix::StreamHandler<Result<Message, ProtocolError>> for EchoSession {
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
                // this is where the echo is happening
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
```

### register the get route

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(echo_route))
        .bind(("127.0.0.1", 9082))?
        .run()
        .await
}
```

### Final code

```rust
use actix::prelude::*;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self, Message, ProtocolError, WebsocketContext};

pub struct EchoSession;

impl Actor for EchoSession {
    type Context = WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<Message, ProtocolError>> for EchoSession {
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

#[get("/ws/echo")]
async fn echo_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let ws_actor = EchoSession {};

    ws::start(ws_actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(echo_route))
        .bind(("127.0.0.1", 9082))?
        .run()
        .await
}
```

### run the server

```zsh
cargo run
```

OR with live reloading

```zsh
watchexec -e rs -r cargo run
```
