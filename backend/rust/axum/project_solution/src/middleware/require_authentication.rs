use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::{
    database::users::{self, Entity as Users},
    errors::app_error::AppError,
    utilities::{jwt::validate_token, token_wrapper::TokenWrapper},
};

pub async fn require_authentication(
    State(db): State<DatabaseConnection>,
    State(token_wrapper): State<TokenWrapper>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))?
        .token()
        .to_owned();

    validate_token(&token_wrapper, &token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(&token))
        .one(&db)
        .await
        .map_err(|_: DbErr| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account",
            )
        })?;

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized to access this resource",
        ));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
