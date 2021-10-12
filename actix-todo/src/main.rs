mod config;
mod models;

use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status { status: "UP" })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
