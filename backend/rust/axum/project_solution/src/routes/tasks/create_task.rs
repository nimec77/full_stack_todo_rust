use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    database::users::Model as UserModel,
    errors::app_error::AppError, queries::task_queries,
};

use super::{create_task_extractor::ValidateCreateTask, ResponseDataTask};

pub async fn create_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;
    let response_data = ResponseDataTask {
        task: task.into(),
    };

    Ok((StatusCode::CREATED, Json(response_data)))
}
