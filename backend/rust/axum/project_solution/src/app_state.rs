use axum::{extract::FromRef, http::Uri};
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub api_url: Uri,
    pub db: DatabaseConnection,
    pub jwt_secret: String,
    pub jwt_expiration_time: i64,
}
