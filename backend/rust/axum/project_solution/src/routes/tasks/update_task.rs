use crate::{
    database::tasks::{self, Column, Entity as Task, },
    errors::app_error::AppError,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use super::RequestTask;

pub async fn atomic_update_task(
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(request_task.priority.unwrap_or(None)),
        title: Set(request_task.title.unwrap()),
        completed_at: Set(request_task.completed_at.unwrap_or(None)),
        description: Set(request_task.description.unwrap_or(None)),
        ..Default::default()
    };

    Task::update(update_task)
        .filter(Column::Id.eq(task_id))
        .exec(&db)
        .await
        .map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    Ok(StatusCode::OK)
}
