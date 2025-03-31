use actix::Addr;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

use crate::{
    actors::{
        chat_manager::message::ChatManager, chat_server::ChatServer,
        chat_session::actor::ChatSession, session::actor::Session,
        session_manager::actor::SessionManager, ws_chat_session::WsChatSession,
    },
    utils::generate_code::generate_random,
};

#[get("/ws")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    println!("here");
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[get("/ws/chat")]
async fn chat_create(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<SessionManager>>,
) -> Result<HttpResponse, Error> {
    let session_actor = Session {
        manager: srv.get_ref().clone(),
        code: None,
    };

    ws::start(session_actor, &req, stream)
}

#[get("/ws/chat/{code}")]
async fn chat_with_code(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<SessionManager>>,
    code: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let session_actor = Session {
        manager: srv.get_ref().clone(),
        code: Some(code.to_string()),
    };

    ws::start(session_actor, &req, stream)
}

#[get("/ws-chat")]
async fn ws_chat(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatManager>>,
) -> Result<HttpResponse, Error> {
    // Ok(format!("Request Body Bytes:\n{:?}", bytes))
    println!("{:?}", req);

    let session_actor = ChatSession {
        manager: srv.get_ref().clone(),
        code: None,
        session_id: generate_random(),
    };

    ws::start(session_actor, &req, stream)
}
