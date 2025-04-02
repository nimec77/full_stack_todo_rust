use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod delete_user;
pub mod login;
pub mod logout;

use crate::database::users::Model as UserModel;

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

