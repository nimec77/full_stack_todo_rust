use crate::database::{tasks, tasks::Entity as Task, tasks::Model as TaskModel};
use axum::extract::{Path, Query};
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

impl From<TaskModel> for ResponseTask {
    fn from(model: TaskModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            priority: model.priority,
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
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Task::find_by_id(task_id).one(&database).await.unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask::from(task)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTaskQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();

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
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_tasks = tasks.into_iter().map(ResponseTask::from).collect();
    Ok(Json(response_tasks))
}
