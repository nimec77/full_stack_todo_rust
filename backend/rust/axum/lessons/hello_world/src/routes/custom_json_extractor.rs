use axum::{
    Json, RequestExt,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must be at least 8 characters long"))]
    pub password: String,
}

impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = req
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{error}")))?;

        if let Err(error) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{error}")));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(&user);
}
