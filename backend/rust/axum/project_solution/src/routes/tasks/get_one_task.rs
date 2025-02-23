use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use crate::database::tasks::Entity as Task;

#[derive(Debug, Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn get_one_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Task::find_by_id(task_id).one(&database).await.unwrap();

    if let Some(task) = task {
        let response_task = ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
        };

        Ok(Json(response_task))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
