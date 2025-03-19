use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod login;
pub mod logout;

use crate::database::users::ActiveModel as UserActiveModel;

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
    username: String,
    token: String,
}

impl From<UserActiveModel> for ResponseUser {
    fn from(user: UserActiveModel) -> Self {
        Self {
            username: user.username.unwrap(),
            token: user.token.unwrap().unwrap(),
        }
    }
}
