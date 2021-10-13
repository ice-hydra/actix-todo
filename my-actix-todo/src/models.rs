use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, PostgresMapper, Deserialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub title: String,
    pub id: i32,
}
#[derive(Serialize, PostgresMapper, Deserialize)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub title: String,
    pub id: i32,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool,
}
