use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestUser {
    username: String,
    password: Option<String>,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>)   {    
    dbg!("username: {}", user.username);
    dbg!("password: {}", user.password.unwrap());
}
