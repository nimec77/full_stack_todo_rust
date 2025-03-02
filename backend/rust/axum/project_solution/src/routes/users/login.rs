use crate::database::{users, users::ActiveModel as UserActiveModel, users::Entity as User};
use axum::{Extension, Json, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    id: i32,
    username: String,
    token: Option<String>,
}

impl From<UserActiveModel> for LoginResponse {
    fn from(user: UserActiveModel) -> Self {
        Self {
            id: user.id.unwrap(),
            username: user.username.unwrap(),
            token: user.token.unwrap(),
        }
    }
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let filter = users::Column::Username
        .eq(user.username)
        .and(users::Column::Password.eq(user.password));

    let user = User::find()
        .filter(filter)
        .one(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        let token = "random_token";
        let mut user = user.into_active_model();
        user.token = Set(Some(token.to_owned()));
        let saved_user = user
            .save(&database)
            .await
            .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(LoginResponse::from(saved_user)))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
