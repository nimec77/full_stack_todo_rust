
use axum::{routing::{get, post}, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::routes::{hello_world::hello_world, tasks::create_task::create_task};

pub fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
}
