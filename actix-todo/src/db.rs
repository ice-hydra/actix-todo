use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let stmt = client
        .prepare("SELECT id,title FROM todo_list ORDER BY id DESC LIMIT 100")
        .await
        .unwrap();
    let todos = client
        .query(&stmt, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let stmt = client
        .prepare(
            "SELECT id, title, checked, list_id FROM todo_item WHERE list_id=$1 ORDER BY id ASC",
        )
        .await
        .unwrap();
    let items = client
        .query(&stmt, &[&list_id])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id,title")
        .await
        .unwrap();

    client
        .query(&stmt, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo list",
        ))
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {
    let stmt = client
        .prepare("UPDATE todo_item SET checked=true WHERE id=$1 AND list_id=$2 AND checked=false")
        .await
        .unwrap();
    let result = client
        .execute(&stmt, &[&item_id, &list_id])
        .await
        .expect("Error checking item");
    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Error checking item")),
    }
}
