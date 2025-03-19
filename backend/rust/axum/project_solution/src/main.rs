use axum::http::Uri;
use dotenvy::dotenv;
use project_solution::{app_state::AppState, run, utilities::token_wrapper::TokenWrapper};
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_url = env::var("API_URL")
        .expect("Missing environment variable API_URL")
        .parse::<Uri>()
        .expect("Invalid API_URL");

    let database_uri = env::var("DATABASE_URL")
        .expect("Missing environment variable DATABASE_URI")
        .to_owned();

    let jwt_secret = env::var("JWT_SECRET")
        .expect("Missing environment variable JWT_SECRET")
        .to_owned();

    let jwt_expiration_time = env::var("JWT_EXPIRATION_TIME")
        .expect("Missing environment variable JWT_EXPIRATION_TIME")
        .parse::<i64>()
        .expect("Invalid JWT_EXPIRATION_TIME");

    let db = match Database::connect(&database_uri).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error connecting to database: {:?}", e);
            panic!("Failed to connect to database");
        }
    };

    let token_wrapper = TokenWrapper {
        secret: jwt_secret,
        expiration_time: jwt_expiration_time,
    };

    let app_state = AppState { db, token_wrapper };

    run(api_url, app_state).await;
}
