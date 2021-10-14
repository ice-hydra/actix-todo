use crate::errors::{AppError, AppErrorType};
use crate::form;
use crate::models;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn create_todo(
    client: &Client,
    frm: &form::CreateTodoForm,
) -> Result<models::TodoList, AppError> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id, title")
        .await
        .map_err(AppError::db_error)?;
    client
        .query(&stmt, &[&frm.title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| models::TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<models::TodoList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating TODO list".to_string()),
            cause: Some("Unknow error".to_string()),
            error_type: AppErrorType::DbError,
        })
}

pub async fn get_todos(client: &Client) -> Result<Vec<models::TodoList>, AppError> {
    let stmt = client
        .prepare("SELECT id, title FROM todo_list ORDER BY id DESC LIMIT 100")
        .await
        .map_err(AppError::db_error)?;
    let result = client
        .query(&stmt, &[])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| models::TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<models::TodoList>>();
    Ok(result)
}

pub async fn check_item(client: &Client, item_id: i32, list_id: i32) -> Result<bool, AppError> {
    let stmt = client
        .prepare("UPDATE todo_item SET checked=true WHERE id=$1 AND list_id=$2 AND checked=false")
        .await
        .map_err(AppError::db_error)?;
    let result = client
        .execute(&stmt, &[&item_id, &list_id])
        .await
        .map_err(AppError::db_error)?;
    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false),
    }
}
