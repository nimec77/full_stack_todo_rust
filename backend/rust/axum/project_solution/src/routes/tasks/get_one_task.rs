use axum::{
    extract::{Path, State}, http::StatusCode, Extension, Json
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::{
        tasks::{self, Entity as Tasks},
        users::Model as UserModel,
    },
    errors::app_error::AppError,
};

use super::ResponseDataTask;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?;

    let task = task.ok_or(AppError::new(
        StatusCode::NOT_FOUND,
        "Task not found",
    ))?;

    Ok(Json(ResponseDataTask {
        task: task.into(),
    }))
}
