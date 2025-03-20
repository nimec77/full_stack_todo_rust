use crate::{app_state::AppState, routes::users::delete_user::delete_user};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post, put},
};

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
    Router::new()
        .route("/api/v1/user", post(create_user))
        .route("/api/v1/user/{id}", delete(delete_user))
        .route("/api/v1/task", post(create_task))
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/task/{id}", get(get_one_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/task/{id}", put(atomic_update_task))
        .route("/api/v1/task/{id}", patch(partial_update_task))
        .route("/api/v1/task/{id}", delete(delete_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users/login", post(login))
        .with_state(app_state)
}
