use axum::{Extension, extract::Path, http::StatusCode};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::database::tasks::Entity as Task;

pub async fn delete_task(
    Extension(database): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<(), StatusCode> {
    Task::delete_by_id(task_id)
        .exec(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
