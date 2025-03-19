use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::utilities::token_wrapper::TokenWrapper;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub token_wrapper: TokenWrapper,
}
