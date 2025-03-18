use axum::{body::Body, http::{Request, Response, StatusCode}, middleware::Next};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::{database::users::{self, Entity as Users}, errors::app_error::AppError, utilities::jwt::validate_token};

pub async fn require_authentication(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let token = request.headers().typed_get::<Authorization<Bearer>>()
    .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "Unauthorized"))?
    .token()
    .to_owned();

    validate_token(&token)?;

    let database = request.extensions().get::<DatabaseConnection>()
    .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    let user = Users::find()
    .filter(users::Column::Token.eq(&token))
    .one(database)
    .await
    .map_err(|_: DbErr| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was a problem getting your account"))?;
    

    let Some(user) = user else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized to access this resource"));
    };

    request.extensions_mut().insert(user);
    
    Ok(next.run(request).await)
}
