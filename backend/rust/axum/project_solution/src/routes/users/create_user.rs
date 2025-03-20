use crate::{
    database::users::ActiveModel as UserModel,
    errors::app_error::AppError,
    utilities::{hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use super::{RequestUser, ResponseDataUser, ResponseUser};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(token_wrapper): State<TokenWrapper>,
    Json(user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let hashed_password = hash_password(&user.password)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let token = create_token(&token_wrapper, &user.username)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let new_user = UserModel {
        username: Set(user.username),
        password: Set(hashed_password),
        token: Set(Some(token)),
        deleted_at: Set(None),
        ..Default::default()
    }
    .save(&db)
    .await
    .map_err(|error: DbErr| {
        println!("Error creating user: {:?}", &error);
        let error_message = error.to_string();
        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(StatusCode::BAD_REQUEST, "Username already taken, try again with a different user name")
        } else {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser::from(new_user),
    }))
}
