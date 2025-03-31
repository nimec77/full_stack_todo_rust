mod routes;
mod router;
mod database;
mod middleware;
pub mod utilities;
pub mod app_state;
pub mod errors;
pub mod queries;

use app_state::AppState;
use axum::http::Uri;
use tokio::net::TcpListener;

use router::create_router;

pub async fn run(app_url: Uri, app_state: AppState) {
    let app = create_router(app_state);

    let address = TcpListener::bind(app_url.to_string()).await.unwrap();
    axum::serve(address, app).await.unwrap();
}
