use crate::{
    database::users::{self, ActiveModel as UserActiveModel, Entity as Users},
    utilities::{hash::verify_password, jwt::create_token},
};
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
    Json(request_user): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = create_token(&user.username)?;
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
