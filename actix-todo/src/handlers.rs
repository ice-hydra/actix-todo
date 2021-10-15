use crate::{db, errors::AppError, models};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use slog::{crit, error, o, Logger};

async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
        AppError::db_error(err)
    })
}

fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
        error!(sublog, "{}", err.message());
        err
    })
}

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status { status: "UP" })
}

pub async fn get_todos(state: web::Data<models::AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    db::get_todos(&client)
        .await
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}

pub async fn get_items(
    state: web::Data<models::AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let path = path.into_inner();
    let log = state.log.new(o!("handler" => "get_items"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    db::get_items(&client, path.0)
        .await
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}

pub async fn create_todo(
    state: web::Data<models::AppState>,
    json: web::Json<models::CreateTodoList>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_todo"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    db::create_todo(&client, json.title.clone())
        .await
        .map(|todo_list| HttpResponse::Ok().json(todo_list))
        .map_err(log_error(log))
}

pub async fn check_item(
    state: web::Data<models::AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let path = path.into_inner();
    let log = state.log.new(o!("handler" => "check_item"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    db::check_item(&client, path.0, path.1)
        .await
        .map(|updated| HttpResponse::Ok().json(models::ResultResponse { success: updated }))
        .map_err(log_error(log))
}
