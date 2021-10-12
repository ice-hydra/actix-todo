use serde::Serialize;
#[derive(Serialize)]
pub struct Status<'a> {
    pub status: &'a str,
}
