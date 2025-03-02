use crate::database::tasks::Entity as Task;
use crate::database::tasks::Model as TaskModel;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use serde::Serialize;

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
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let tasks = Task::find()
        .all(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_tasks = tasks
        .into_iter()
        .map(ResponseTask::from)
        .collect();
    Ok(Json(response_tasks))
}
