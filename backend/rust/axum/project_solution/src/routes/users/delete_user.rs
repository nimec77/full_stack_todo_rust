use crate::{
    database::users::Model as UserModel,
    errors::app_error::AppError,
    queries::user_queries,
};
use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::DatabaseConnection;

pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {
    user_queries::delete_user(&db, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}
