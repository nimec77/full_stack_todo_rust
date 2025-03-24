use crate::database::{tasks, tasks::Entity as Task};
use crate::errors::app_error::AppError;
use axum::extract::{Path, Query, State};
use axum::{Json, http::StatusCode};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::Deserialize;

use super::ResponseTask;


#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

pub async fn get_one_task(
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseTask>, AppError> {
    let task = Task::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask::from(task)))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"))
    }
}

pub async fn get_all_tasks(
    State(db): State<DatabaseConnection>,
    Query(query_params): Query<GetTaskQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, AppError> {
    let mut priority_filter = Condition::all().add(tasks::Column::DeletedAt.is_null());

    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    let tasks = Task::find()
        .filter(priority_filter)
        .all(&db)
        .await
        .map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
    let response_tasks = tasks.into_iter().map(ResponseTask::from).collect();
    Ok(Json(response_tasks))
}
