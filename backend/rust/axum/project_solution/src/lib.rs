mod routes;
mod router;
mod database;
mod middleware;
mod utilities;
pub mod app_state;
pub mod errors;
use app_state::AppState;
use tokio::net::TcpListener;

use router::create_router;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);

    let address = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    axum::serve(address, app).await.unwrap();
}
