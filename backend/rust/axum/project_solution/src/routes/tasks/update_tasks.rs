use axum::{
    extract::{Path, State}, http::StatusCode, Extension, Json
};
use chrono::Utc;
use sea_orm::{
    ActiveValue::Set, DatabaseConnection,
    IntoActiveModel,
};

use crate::{
    database::users::Model as UserModel,
    errors::app_error::AppError, queries::task_queries,
};

use super::RequestTask;

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id).await?
    .into_active_model();

    let now = Utc::now();
    task.completed_at = Set(Some(now.into()));

    task_queries::save_active_task(&db, task).await?;

    Ok(StatusCode::OK)
}

pub async fn mark_uncompleted(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id).await?
    .into_active_model();

    task.completed_at = Set(None);

    task_queries::save_active_task(&db, task).await?;

    Ok(StatusCode::OK)
}

pub async fn update_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id).await?
    .into_active_model();


    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        task.title = Set(title);
    }

    if let Some(description) = request_task.description {
        task.description = Set(description);
    }

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    task_queries::save_active_task(&db, task).await?;

    Ok(StatusCode::OK)
}
