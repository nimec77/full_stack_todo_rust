use crate::database::users::ActiveModel as UserModel;
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct CreateUserResponse {
    id: i32,
    username: String,
    token: Option<String>,
}

impl From<UserModel> for CreateUserResponse {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id.unwrap(),
            username: user.username.unwrap(),
            token: user.token.unwrap(),
        }
    }
}
pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let new_user = UserModel {
        username: Set(user.username),
        password: Set(user.password),
        token: Set(None),
        deleted_at: Set(None),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateUserResponse::from(new_user)))
}
