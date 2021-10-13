use serde::Deserialize;
#[derive(Deserialize)]
pub struct CreateTodoForm {
    pub title: String,
}
