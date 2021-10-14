use crate::db;
use crate::errors::AppError;
use crate::form;
use crate::models;
use actix_web::{web, Responder};
use deadpool_postgres::{Client, Pool};
use slog::Logger;
use slog::{crit, error, o};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status {
        status: "UP".to_string(),
    })
}

async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error getting client");
        AppError::db_error(err)
    })
}

fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
        error!(sublog, "db error");
        AppError::db_error(err)
    })
}

pub async fn create_todo(
    state: web::Data<models::AppState>,
    json: web::Json<form::CreateTodoForm>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=>"create_todo"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    db::create_todo(&client, &json)
        .await
        .map(|todos| web::HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

pub async fn get_todos(state: web::Data<models::AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=>"get_todos"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    db::get_todos(&client)
        .await
        .map(|todos| web::HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

pub async fn check_item(
    state: web::Data<models::AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=>"check_item"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    db::check_item(&client, path.1, path.0)
        .await
        .map(|update| web::HttpResponse::Ok().json(models::ResultResponse { success: update }))
        .map_err(log_error(log))
}
