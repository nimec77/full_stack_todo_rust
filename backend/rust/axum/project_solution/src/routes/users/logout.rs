use axum::{Extension, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel, Set};

use crate::{database::users::Model, errors::app_error::AppError};

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<StatusCode, AppError> {
    let mut user_model = user.into_active_model();
    user_model.token = Set(None);

    user_model.save(&database).await.map_err(|_: DbErr| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(StatusCode::OK)
}
