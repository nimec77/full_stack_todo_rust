use axum::{Extension, extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{
    database::users::Model as UserModel, errors::app_error::AppError, queries::user_queries,
};

pub async fn logout(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<StatusCode, AppError> {
    let mut user_model = user.into_active_model();
    user_model.token = Set(None);

    user_queries::save_active_user(&db, user_model).await?;

    Ok(StatusCode::OK)
}
