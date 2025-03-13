use axum::{Extension, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::database::users::{self, Entity as Users};

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
) -> Result<StatusCode, StatusCode> {
    let token = authorization.token();
    let mut user_model = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user.into_active_model()
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    user_model.token = Set(None);

    user_model
        .save(&database)
        .await
        .map_err(|_: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
