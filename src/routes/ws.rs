use actix_web::{web, get, Error, HttpRequest, HttpResponse};
use std::{sync::Arc, time::Instant};
use actix_web_actors::ws;

// #[get("/ws")]
// async fn chat_route(
//     req: HttpRequest,
//     stream: web::Payload,
//     // srv: web::Data<Addr<server::ChatServer>>,
// ) -> Result<HttpResponse, Error> {
//     ws::start(
//         // session::WsChatSession {
//         //     id: 0,
//         //     hb: Instant::now(),
//         //     room: "main".to_owned(),
//         //     name: None,
//         //     addr: srv.get_ref().clone(),
//         // },
//         &req, stream,
//     )
// }
