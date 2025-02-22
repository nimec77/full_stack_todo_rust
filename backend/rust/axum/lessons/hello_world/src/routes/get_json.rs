use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    pub message: String,
    pub count: i32,
    pub username: String,
}

pub async fn get_json() -> Json<Data> {
    let data =Data {
        message: "Hello, world!".to_owned(),
        count: 42,
        username: "John Doe".to_owned(),
    };

    Json(data)
}
