pub mod config;
pub mod models;
pub mod db;

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
    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 9081))?
    .run()
    .await
}