use dotenvy::dotenv;
use project_solution::run;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_uri = env::var("DATABASE_URL")
        .expect("Missing environment variable DATABASE_URI")
        .to_owned();

    run(&database_uri).await;
}
