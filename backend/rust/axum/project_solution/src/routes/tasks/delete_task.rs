use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;

use crate::{database::tasks::Entity as Task, errors::app_error::AppError};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Query(query_params): Query<QueryParams>,
) -> Result<StatusCode, AppError> {
    if query_params.soft {
        let mut task = if let Some(task) =
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
        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        task.update(&db).await.map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
    } else {
        Task::delete_by_id(task_id)
            .exec(&db)
            .await
            .map_err(|_: DbErr| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            })?;
    }
    Ok(StatusCode::NO_CONTENT)
}
