mod router;
mod routes;

use router::create_router;
use sea_orm::Database;
use tokio::net::TcpListener;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = create_router(database);

    let address = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    axum::serve(address, app).await.unwrap();
}
