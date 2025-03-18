use std::env;

use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};

use crate::errors::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(username: &str) -> Result<String, StatusCode> {
    let now = chrono::Utc::now();
    let expired_at = now + Duration::seconds(3600);
    let exp = expired_at.timestamp() as usize;

    let claims = Claims {
        exp,
        username: username.to_string(),
    };
    let token_header = Header::default();
    let jwt_secret = if let Ok(secret) = env::var("JWT_SECRET") {
        secret
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error encoding token: {:?}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub fn validate_token(token: &str) -> Result<bool, AppError> {
    let jwt_secret = if let Ok(secret) = env::var("JWT_SECRET") {
        secret
    } else {
        return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"));
    };

    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            ErrorKind::ExpiredSignature | ErrorKind::InvalidToken | ErrorKind::InvalidSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated")
            }
            _ => {
                eprintln!("Error verifying token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })
        .map(|_| true)
}
