// api/src/main.rs
use axum::{Router, routing::get};
use tokio::net::TcpListener;

// ➊ declare the module (looks for src/response/mod.rs)
mod app_state;
mod extract;
mod response;
mod routes;
use routes::users_routes;

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
        .route("/health", get(health))
        .merge(users_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
