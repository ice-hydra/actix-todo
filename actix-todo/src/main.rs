mod models;
use actix_web::{web, App, HttpServer, Responder};
async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status { status: "OK" })
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
