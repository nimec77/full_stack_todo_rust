use crate::database::{tasks, tasks::Entity as Task, tasks::Model as TaskModel};
use crate::errors::app_error::AppError;
use axum::extract::{Path, Query};
use axum::{Extension, Json, http::StatusCode};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

impl From<TaskModel> for ResponseTask {
    fn from(task: TaskModel) -> Self {
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

pub async fn get_one_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseTask>, AppError> {
    let task = Task::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask::from(task)))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "Task not found"))
    }
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
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
        .all(&database)
        .await
        .map_err(|_: DbErr| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let response_tasks = tasks.into_iter().map(ResponseTask::from).collect();
    Ok(Json(response_tasks))
}
