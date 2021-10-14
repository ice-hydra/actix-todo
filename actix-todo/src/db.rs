use crate::{
    errors::{AppError, AppErrorType},
    models::{TodoItem, TodoList},
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError> {
    let stmt = client
        .prepare("SELECT id,title FROM todo_list ORDER BY id DESC LIMIT 100")
        .await
        .map_err(AppError::db_error)?;
    let todos = client
        .query(&stmt, &[])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, AppError> {
    let stmt = client
        .prepare(
            "SELECT id, title, checked, list_id FROM todo_item WHERE list_id=$1 ORDER BY id ASC",
        )
        .await
        .map_err(AppError::db_error)?;
    let items = client
        .query(&stmt, &[&list_id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, AppError> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id,title")
        .await
        .map_err(AppError::db_error)?;

    client
        .query(&stmt, &[&title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating TODO list".to_string()),
            cause: Some("Unknow error".to_string()),
            error_type: AppErrorType::DbError,
        })
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<bool, AppError> {
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
