use axum::{Extension, Json, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde::Deserialize;

use crate::database::tasks;
use crate::database::users::{self, Entity as User};

#[derive(Debug, Deserialize)]
pub struct RequestTask {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    Json(request_task): Json<RequestTask>,
) -> Result<StatusCode, StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = User::find()
        .filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_: DbErr| StatusCode::UNAUTHORIZED)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    new_task
        .save(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
