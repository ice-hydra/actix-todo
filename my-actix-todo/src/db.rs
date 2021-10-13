use crate::form;
use crate::models;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn create_todo(
    client: &Client,
    frm: &form::CreateTodoForm,
) -> Result<models::TodoList, io::Error> {
    let stmt = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1) RETURNING id, title")
        .await
        .unwrap();
    client
        .query(&stmt, &[&frm.title])
        .await
        .unwrap()
        .iter()
        .map(|row| models::TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<models::TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Creating todo failed"))
}

pub async fn get_todos(client: &Client) -> Result<Vec<models::TodoList>, io::Error> {
    let stmt = client
        .prepare("SELECT id, title FROM todo_list ORDER BY id DESC LIMIT 100")
        .await
        .unwrap();
    let result = client
        .query(&stmt, &[])
        .await
        .unwrap()
        .iter()
        .map(|row| models::TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<models::TodoList>>();
    Ok(result)
}

pub async fn check_item(client: &Client, item_id: i32, list_id: i32) -> Result<(), io::Error> {
    let stmt = client
        .prepare("UPDATE todo_item SET checked=true WHERE id=$1 AND list_id=$2 AND checked=false")
        .await
        .unwrap();
    let result = client.execute(&stmt, &[&item_id, &list_id]).await.unwrap();
    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "checking item failed")),
    }
}
