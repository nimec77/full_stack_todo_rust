use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use sea_orm::{
    ActiveValue::Set, DatabaseConnection,
    IntoActiveModel,
};

use crate::{
    database::users::Model as UserModel,
    errors::app_error::AppError,
    queries::task_queries,
};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {
    let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));

    task_queries::save_active_task(&db, task).await?;

    Ok(StatusCode::NO_CONTENT)
}
