use axum::{
    Extension, Router,
    routing::{delete, get, patch, post, put},
};
use sea_orm::DatabaseConnection;

use crate::routes::{
    hello_world::hello_world,
    tasks::{
        create_task::create_task,
        delete_task::delete_task,
        get_tasks::{get_all_tasks, get_one_task},
        partial_update_task::partial_update_task,
        update_task::atomic_update_task,
    },
    users::{create_user::create_user, login::login, logout::logout},
};

pub fn create_router(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/task", post(create_task))
        .route("/task/{id}", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .route("/task/{id}", put(atomic_update_task))
        .route("/task/{id}", patch(partial_update_task))
        .route("/task/{id}", delete(delete_task))
        .route("/user", post(create_user))
        .route("/users/login", post(login))
        .route("/users/logout", post(logout))
        .layer(Extension(database))
}
