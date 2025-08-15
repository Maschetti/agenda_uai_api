// api/src/main.rs
use axum::{routing::get, Router};
use tokio::net::TcpListener;

// ➊ declare the module (looks for src/response/mod.rs)
mod response;

// ➋ bring types into scope
use crate::response::ApiResponse;

#[derive(serde::Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> ApiResponse<Health> {
    ApiResponse {
        status_code: 200,
        success: true,
        data: Some(Health { status: "ok" }),
        errors: None,
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello from API" }))
        .route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
