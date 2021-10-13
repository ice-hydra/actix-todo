use crate::db;
use crate::form;
use crate::models;
use actix_web::{web, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status {
        status: "UP".to_string(),
    })
}

pub async fn create_todo(
    pool: web::Data<Pool>,
    json: web::Json<form::CreateTodoForm>,
) -> impl Responder {
    let client: Client = pool.get().await.unwrap();
    let result = db::create_todo(&client, &json).await;
    match result {
        Ok(todo) => web::HttpResponse::Ok().json(todo),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_todos(pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pool.get().await.unwrap();
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => web::HttpResponse::Ok().json(todos),
        Err(_) => web::HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_item(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = pool.get().await.unwrap();
    let result = db::check_item(&client, path.1, path.0).await;
    match result {
        Ok(()) => web::HttpResponse::Ok().json(models::ResultResponse { success: true }),
        Err(e) if e.kind() == std::io::ErrorKind::Other => {
            web::HttpResponse::Ok().json(models::ResultResponse { success: false })
        }
        _ => web::HttpResponse::InternalServerError().into(),
    }
}
