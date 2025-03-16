use axum::{
    Extension, Router, middleware,
    routing::{delete, get, patch, post, put},
};
use crate::app_state::AppState;

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

use crate::middleware::require_authentication::require_authentication;

pub fn create_router(app_state: AppState) -> Router {
    let db = app_state.db;
    Router::new()
        .route("/user", post(create_user))
        .route("/task", post(create_task))
        .route("/users/logout", post(logout))
        .route("/task/{id}", get(get_one_task))
        .route("/tasks", get(get_all_tasks))
        .route("/task/{id}", put(atomic_update_task))
        .route("/task/{id}", patch(partial_update_task))
        .route("/task/{id}", delete(delete_task))
        .route_layer(middleware::from_fn(require_authentication))
        .route("/", get(hello_world))
        .route("/users/login", post(login))
        .layer(Extension(db))
}
