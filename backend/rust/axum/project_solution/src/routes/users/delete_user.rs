use crate::{database::users::Entity as Users, errors::app_error::AppError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    Users::delete_by_id(user_id)
        .exec(&db)
        .await
        .map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    Ok(StatusCode::NO_CONTENT)
}
