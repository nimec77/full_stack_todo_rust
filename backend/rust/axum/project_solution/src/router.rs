use axum::{
    Extension, Router,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;

use crate::routes::{
    hello_world::hello_world,
    tasks::{
        create_task::create_task,
        get_tasks::{get_all_tasks, get_one_task},
        update_task::atomic_update_task,
    },
};

pub fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/task", post(create_task))
        .route("/task/{id}", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .route("/task/{id}", put(atomic_update_task))
        .layer(Extension(database))
}
