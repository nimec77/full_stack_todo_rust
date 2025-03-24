use std::borrow::Cow;

use axum::{
    Json, RequestExt,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
};
use serde::Deserialize;
use validator::Validate;

use crate::errors::app_error::AppError;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateTask {
    #[validate(length(min = 1, max = 1, message = "Priority must be one letter"))]
    pub priority: Option<String>,
    #[validate(required(message = "Missing task title"))]
    pub title: Option<String>,
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,
}

impl<S> FromRequest<S> for ValidateCreateTask
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = req
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprint!("Error extracting new task: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again later",
                )
            })?;

        if let Err(errors) = task.validate() {
            let field_errors = errors.field_errors();
            let (_, error) = field_errors
                .into_iter()
                .next()
                .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "Invalid input"))?;
            let error_message = error
                .first()
                .unwrap()
                .to_owned()
                .message
                .unwrap_or(Cow::Borrowed("Invalid input"));
            return Err(AppError::new(StatusCode::BAD_REQUEST, error_message));
        }

        Ok(task)
    }
}
