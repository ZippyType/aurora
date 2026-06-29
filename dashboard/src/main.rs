mod monitor;

use axum::{
    Router,
    routing::get,
    response::Json,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/cpu", get(monitor::cpu::handler))
        .route("/api/memory", get(monitor::memory::handler))
        .route("/api/storage", get(monitor::storage::handler))
        .route("/api/battery", get(monitor::battery::handler))
        .route("/api/temperature", get(monitor::temperature::handler))
        .route("/api/processes", get(monitor::processes::handler))
        .route("/api/network", get(monitor::network::handler))
        .fallback_service(ServeDir::new("web"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Aurora dashboard listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
