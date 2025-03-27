use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users::Model as UserModel;
use crate::database::tasks::{self, Entity as Tasks};
use crate::errors::app_error::AppError;

use super::ResponseDataTaskList;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTaskList>, AppError> {
    let tasks = Tasks::find()
        .filter(tasks::Column::UserId.eq(user.id))
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting all tasks {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    let tasks = tasks.into_iter().map(|task| task.into()).collect();

    Ok(Json(ResponseDataTaskList { tasks }))
}
