use crate::{database::tasks::Entity as Task, errors::app_error::AppError};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set,
};

use super::RequestTask;

pub async fn partial_update_task(
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let mut db_task = if let Some(task) =
        Task::find_by_id(task_id)
            .one(&db)
            .await
            .map_err(|_: DbErr| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            })? {
        task.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"));
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        db_task.description = Set(description);
    }

    // Update the task in the database
    Task::update(db_task).exec(&db).await.map_err(|_: DbErr| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(StatusCode::OK)
}
