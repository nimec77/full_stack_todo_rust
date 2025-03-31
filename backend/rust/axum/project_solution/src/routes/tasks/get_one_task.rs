use axum::{
    Extension, Json,
    extract::{Path, State},
};
use sea_orm::DatabaseConnection;

use crate::{
    database::users::Model as UserModel,
    errors::app_error::AppError,
    queries::task_queries,
};

use super::ResponseDataTask;

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = task_queries::find_task_by_id(&db, task_id, user.id).await?;

    Ok(Json(ResponseDataTask { task: task.into() }))
}
