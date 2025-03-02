
use axum::{routing::{get, post}, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::routes::{hello_world::hello_world, tasks::{create_task::create_task, get_tasks::{get_all_tasks, get_one_task}}};

pub fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/task", post(create_task))
        .route("/task/{id}", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .layer(Extension(database))
}
