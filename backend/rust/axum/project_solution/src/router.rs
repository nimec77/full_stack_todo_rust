use crate::{
    app_state::AppState,
    routes::{
        tasks::{
            get_all_tasks::get_all_tasks,
            get_one_task::get_one_task,
            update_tasks::{mark_completed, mark_uncompleted, update_task},
        },
        users::delete_user::delete_user,
    },
};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post, put},
};

use crate::routes::{
    hello_world::hello_world,
    tasks::{create_task::create_task, delete_task::delete_task},
    users::{create_user::create_user, login::login, logout::logout},
};

use crate::middleware::require_authentication::require_authentication;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/user/{id}", delete(delete_user))
        .route("/api/v1/task", post(create_task))
        .route("/api/v1/users/logout", post(logout))
        .route("/api/v1/task/{id}", get(get_one_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks/{task_id}/completed", put(mark_completed))
        .route("/api/v1/tasks/{task_id}/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/{task_id}", patch(update_task))
        .route("/api/v1/task/{id}", delete(delete_task))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,
        ))
        .route("/", get(hello_world))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/user", post(create_user))
        .with_state(app_state)
}
