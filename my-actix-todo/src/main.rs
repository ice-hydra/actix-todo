mod config;
mod db;
mod form;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();
    println!("Starting at http://{}", cfg.server.addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/todos", web::get().to(handlers::get_todos))
            .route("/todos", web::post().to(handlers::create_todo))
            .route(
                "/todos/{list_id}/items/{item_id}",
                web::put().to(handlers::check_item),
            )
            .route("/", web::get().to(handlers::status))
    })
    .bind(cfg.server.addr)
    .unwrap()
    .run()
    .await
}
