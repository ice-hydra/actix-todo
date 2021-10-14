use crate::{db, errors::AppError, models};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status { status: "UP" })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

    db::get_todos(&client)
        .await
        .map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_items(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

    db::get_items(&client, path.0)
        .await
        .map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_todo(
    db_pool: web::Data<Pool>,
    json: web::Json<models::CreateTodoList>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;

    db::create_todo(&client, json.title.clone())
        .await
        .map(|todo_list| HttpResponse::Ok().json(todo_list))
}

pub async fn check_item(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    db::check_item(&client, path.0, path.1)
        .await
        .map(|updated| HttpResponse::Ok().json(models::ResultResponse { success: updated }))
}
