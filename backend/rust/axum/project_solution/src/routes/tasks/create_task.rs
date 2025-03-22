use axum::{Extension, Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::{
    database::{tasks, users::Model},
    errors::app_error::AppError,
};

use super::RequestTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority.unwrap_or(None)),
        title: Set(request_task.title.unwrap()),
        description: Set(request_task.description.unwrap_or(None)),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task.save(&db).await.map_err(|_: DbErr| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(StatusCode::CREATED)
}
