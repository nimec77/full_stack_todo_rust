use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use axum::http::StatusCode;

pub fn hash_password(password: &str) -> Result<String, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let key = argon2.hash_password(password.as_bytes(), &salt)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(key.to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, StatusCode> {
    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(hashed_password)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    argon2.verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|_: Error| StatusCode::UNAUTHORIZED)?;

    Ok(true)
}
