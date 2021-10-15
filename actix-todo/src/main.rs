mod config;
mod db;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use slog::{info, o, Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};
use tokio_postgres::NoTls;

fn configure_log() -> Logger {
    let decorator = TermDecorator::new().build();
    let console_drain = FullFormat::new(decorator).build().fuse();
    let console_drain = Async::new(console_drain).build().fuse();
    Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    let log = configure_log();

    info!(
        log,
        "Starting server at http://{}:{}", config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(models::AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
            .route("/", web::get().to(handlers::status))
            .route("/todos{_:/?}", web::get().to(handlers::get_todos))
            .route(
                "/todos/{list_id}/items{_:/?}",
                web::get().to(handlers::get_items),
            )
            .route("/todos{_:/?}", web::post().to(handlers::create_todo))
            .route(
                "/todos/{list_id}/items/{item_id}{_:/?}",
                web::put().to(handlers::check_item),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
