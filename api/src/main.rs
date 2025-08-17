// src/main.rs
use axum::{
    Router,
    middleware::{self},
    routing::{get},
};
mod app_state;
mod extract;
mod request;
mod response;
mod routes;
mod middlewares;
use crate::{middlewares::auth::{auth_middleware}, routes::users_routes};
use infra::{JwtService};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::{app_state::AppState};
#[derive(Deserialize)]
struct LoginReq {
    username: String,
}


#[tokio::main]
async fn main() {
    // carregar .env e secret
    let _ = dotenvy::dotenv();
    let secret = std::env::var("JWT_SECRET").expect("defina JWT_SECRET no .env");
    let state = AppState::new(JwtService::new(secret.as_bytes()));

    // rotas públicas
    let public = Router::new()
        .route("/", get(|| async { "Hello from API" }));

    // let protected = Router::new()
    //     .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let users = users_routes(state.clone());
    // app final com estado compartilhado
    let app = Router::new()
        .merge(public)
        .merge(users)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
