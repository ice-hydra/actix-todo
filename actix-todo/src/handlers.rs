use crate::{db, models};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(models::Status { status: "UP" })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(
    db_pool: web::Data<Pool>,
    json: web::Json<models::CreateTodoList>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo(&client, json.title.clone()).await;
    match result {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::check_item(&client, path.0, path.1).await;
    match result {
        Ok(()) => HttpResponse::Ok().json(models::ResultResponse { success: true }),
        Err(ref e) if e.kind() == std::io::ErrorKind::Other => {
            HttpResponse::Ok().json(models::ResultResponse { success: false })
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
