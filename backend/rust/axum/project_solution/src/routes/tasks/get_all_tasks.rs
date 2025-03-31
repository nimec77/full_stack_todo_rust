use axum::{extract::State, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::database::users::Model as UserModel;
use crate::errors::app_error::AppError;
use crate::queries::task_queries;

use super::ResponseDataTaskList;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTaskList>, AppError> {
    let tasks = task_queries::get_all_tasks(&db, user.id, false).await?;

    let tasks = tasks.into_iter().map(|task| task.into()).collect();

    Ok(Json(ResponseDataTaskList { tasks }))
}
