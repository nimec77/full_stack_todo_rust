use crate::{
    database::users::{self, Entity as Users},
    errors::app_error::AppError,
    utilities::{hash::verify_password, jwt::create_token, token_wrapper::TokenWrapper},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use super::{convert_active_to_model, RequestUser, ResponseDataUser, ResponseUser};

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(token_wrapper): State<TokenWrapper>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&db)
        .await
        .map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    if let Some(user) = user {
        if !verify_password(&request_user.password, &user.password)
            .map_err(|_| AppError::new(StatusCode::UNAUTHORIZED, "Invalid username or password"))?
        {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid username or password",
            ));
        }

        let token = create_token(&token_wrapper, &user.username).map_err(|_| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
        let mut user = user.into_active_model();
        user.token = Set(Some(token.to_owned()));
        let saved_user = user.save(&db).await.map_err(|_: DbErr| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;
        let user = convert_active_to_model(saved_user)?;
        Ok(Json(ResponseDataUser {
            data: ResponseUser::from(user),
        }))
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password",
        ))
    }
}
