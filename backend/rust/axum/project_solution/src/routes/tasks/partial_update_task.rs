use crate::database::tasks::Entity as Task;
use axum::{Extension, Json, extract::Path, http::StatusCode};
use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set, prelude::DateTimeWithTimeZone,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestTask {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Task::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
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

    if let Some(deleted_at) = request_task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    // Update the task in the database
    Task::update(db_task)
        .exec(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
