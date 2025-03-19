use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::Deserialize;

use crate::{database::{tasks, users::Model}, errors::app_error::AppError};

#[derive(Debug, Deserialize)]
pub struct RequestTask {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task
        .save(&db)
        .await
        .map_err(|_: DbErr| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    Ok(StatusCode::CREATED)
}
