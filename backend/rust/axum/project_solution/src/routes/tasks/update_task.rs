use crate::{database::tasks::{self, Column, Entity as Task}, errors::app_error::AppError};
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    prelude::DateTimeWithTimeZone,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

pub async fn atomic_update_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        completed_at: Set(request_task.completed_at),
        description: Set(request_task.description),
        deleted_at: Set(request_task.deleted_at),
        user_id: Set(request_task.user_id),
        is_default: Set(request_task.is_default),
    };

    Task::update(update_task)
        .filter(Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_: DbErr| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    Ok(StatusCode::OK)
}
