use axum::{Extension, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use crate::{
    database::{tasks, users::Model},
    errors::app_error::AppError,
};

use super::create_task_extractor::ValidateCreateTask;

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
    task: ValidateCreateTask,
) -> Result<StatusCode, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task.save(&db).await.map_err(|_: DbErr| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(StatusCode::CREATED)
}
