use axum::{
    routing::{get, patch, post, put}, Extension, Router
};
use sea_orm::DatabaseConnection;

use crate::routes::{
    hello_world::hello_world,
    tasks::{
        create_task::create_task,
        get_tasks::{get_all_tasks, get_one_task},
        update_task::atomic_update_task,
        partial_update_task::partial_update_task,
    },
};

pub fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/task", post(create_task))
        .route("/task/{id}", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .route("/task/{id}", put(atomic_update_task))
        .route("/task/{id}", patch(partial_update_task))
        .layer(Extension(database))
}
