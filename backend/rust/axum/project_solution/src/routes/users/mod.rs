use axum::http::StatusCode;
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod delete_user;
pub mod login;
pub mod logout;

use crate::{
    database::users::{ActiveModel as UserActiveModel, Model as UserModel},
    errors::app_error::AppError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

impl From<UserModel> for ResponseUser {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            username: user.username,
            token: user.token.unwrap(),
        }
    }
}

fn convert_active_to_model(active_user: UserActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
