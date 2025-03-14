use crate::{database::users::ActiveModel as UserModel, utilities::hash::hash_password};
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
    let hashed_password = hash_password(&user.password)?;
    let new_user = UserModel {
        username: Set(user.username),
        password: Set(hashed_password),
        token: Set(None),
        deleted_at: Set(None),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|error: DbErr| {
        println!("Error creating user: {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(CreateUserResponse::from(new_user)))
}
