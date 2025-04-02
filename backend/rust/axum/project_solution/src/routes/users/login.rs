use crate::{
    errors::app_error::AppError,
    queries::user_queries,
    utilities::{hash::verify_password, jwt::create_token, token_wrapper::TokenWrapper},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use super::{RequestUser, ResponseDataUser, ResponseUser};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(token_wrapper): State<TokenWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = user_queries::find_user_by_username(&db, &request_user.username).await?;

    if !verify_password(&request_user.password, &user.password)
        .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Invalid username or password"))?
    {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password",
        ));
    }

    let token = create_token(&token_wrapper, &user.username)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let mut user = user.into_active_model();
    user.token = Set(Some(token.to_owned()));
    let saved_user = user_queries::save_active_user(&db, user).await?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser::from(saved_user),
    }))
}
