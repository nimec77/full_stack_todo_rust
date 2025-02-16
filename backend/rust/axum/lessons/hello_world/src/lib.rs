mod routes;

use tokio::net::TcpListener;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
