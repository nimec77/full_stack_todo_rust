use sea_orm::Database;

pub async fn run(database_url: &str) {
    let pool = Database::connect(database_url).await;
}
