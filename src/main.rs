pub mod actors;
pub mod config;
pub mod db;
pub mod messages;
pub mod models;
pub mod routes;
pub mod utils;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actors::{
    chat_manager::message::ChatManager, chat_server::ChatServer, my_actor,
    session_manager::actor::SessionManager,
};
use messages::my_actor::Ping;
use std::sync::{atomic::AtomicUsize, Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start a new actor
    let addr = my_actor::MyActor { count: 11 }.start();

    // send message and get future for result
    let res = addr.send(Ping(10)).await;

    println!("RESULT: {:?}", res.unwrap());

    let pool = config::get_db_pool().await;

    let visitor_count = Arc::new(AtomicUsize::new(0));
    let chat_server = ChatServer::new(visitor_count).start();
    let session_manager = SessionManager::new().start();
    let chat_manager = ChatManager::new().start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(session_manager.clone()))
            .app_data(web::Data::new(chat_manager.clone()))
            .service(routes::user::create_user)
            .service(routes::user::get_user)
            .service(routes::ws::chat_route)
            .service(routes::ws::chat_create)
            .service(routes::ws::chat_with_code)
            .service(routes::ws::ws_chat)
    })
    .bind(("localhost", 9081))?
    .run()
    .await
}
