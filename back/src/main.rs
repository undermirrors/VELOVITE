use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("API Server is listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
