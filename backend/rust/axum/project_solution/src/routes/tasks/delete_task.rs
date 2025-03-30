use axum::{
    extract::{Path, State},
    http::StatusCode, Extension,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model as UserModel,
    },
    errors::app_error::AppError,
};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {

    let mut task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?
        .ok_or(AppError::new(StatusCode::NOT_FOUND, "Task not found"))?
        .into_active_model();
    

        let now = Utc::now();
        task.deleted_at = Set(Some(now.into()));
    
        task.update(&db).await.map_err(|error| {
            eprintln!("Error updating task: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error updating your task",
            )
        })?;
    
    Ok(StatusCode::NO_CONTENT)
}
