use serde::{Deserialize, Serialize};

pub mod create_user;
pub mod login;
pub mod logout;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    username: String,
    token: Option<String>,
}
