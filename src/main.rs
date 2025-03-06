pub mod config;
pub mod models;
pub mod db;
pub mod utils;
pub mod routes;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Info {
    username: String,
}

async fn index(info: web::Json<Info>) -> impl Responder {
    let pool = config::get_db_pool().await;

    let res = db::session::create_session(&pool, "helrelo").await.unwrap();

    println!("res {:?}", res);
    println!("info.username {:?}", info.username);

    HttpResponse::Ok().json(json!({
        "message": res.id,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = config::get_db_pool().await;

    // to force the closure to take ownership of `pool` (and any other referenced variables), use the `move` keyword: `move `
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(web::resource("/").route(web::post().to(index)))
        .service(routes::session::create_session)
    })
    .bind(("127.0.0.1", 9081))?
    .run()
    .await
}