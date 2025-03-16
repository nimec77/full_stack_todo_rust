use dotenvy::dotenv;
use project_solution::{app_state::AppState, run};
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    let app_state = AppState {
        db,
        jwt_secret,
        jwt_expiration_time,
    };

    run(app_state).await;
}
