use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn index(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": format!("Welcome {}!", info.username),
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