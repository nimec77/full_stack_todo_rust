use crate::{
    database::users::ActiveModel as UserModel,
    errors::app_error::AppError,
    utilities::{hash::hash_password, jwt::create_token},
};
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use super::{RequestCreateUser, ResponseDataUser, ResponseUser};

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let hashed_password = hash_password(&user.password)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let token = create_token(&user.username)
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;
    let new_user = UserModel {
        username: Set(user.username),
        password: Set(hashed_password),
        token: Set(Some(token)),
        deleted_at: Set(None),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|error: DbErr| {
        println!("Error creating user: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    Ok(Json(ResponseDataUser {
        data: ResponseUser {
            username: new_user.username.unwrap(),
            token: new_user.token.unwrap(),
        },
    }))
}
