pub mod actors;
pub mod config;
pub mod db;
pub mod messages;
pub mod models;
pub mod routes;
pub mod utils;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use actors::my_actor;
use messages::my_actor::Ping;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start a new actor
    let addr = my_actor::MyActor { count: 11 }.start();

    // send message and get future for result
    let res = addr.send(Ping(10)).await;

    println!("RESULT: {:?}", res.unwrap());

    let pool = config::get_db_pool().await;

    // to force the closure to take ownership of `pool` (and any other referenced variables), use the `move` keyword: `move `
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::session::create_session)
            .service(routes::session::get_session)
    })
    .bind(("127.0.0.1", 9081))?
    .run()
    .await
}
