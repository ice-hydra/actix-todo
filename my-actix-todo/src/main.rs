mod config;
mod db;
mod errors;
mod form;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use slog::{info, o, Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

fn configure_log() -> Logger {
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    let console_drain = Async::new(console_drain).build().fuse();
    Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let pool = cfg.pg.create_pool(tokio_postgres::NoTls).unwrap();
    let log = configure_log();
    info!(log, "Starting at http://{}", cfg.server.addr);

    HttpServer::new(move || {
        App::new()
            .data(models::AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
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
