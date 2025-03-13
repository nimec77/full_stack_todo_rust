use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::Deserialize;

use crate::database::{tasks, users::Model};

#[derive(Debug, Deserialize)]
pub struct RequestTask {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, StatusCode> {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task
        .save(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
