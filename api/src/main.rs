// src/main.rs
use axum::{
    Json, Router,
    extract::{Request, State},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
mod app_state;
mod extract;
mod request;
mod response;
mod routes;
use crate::routes::users_routes;
use infra::{Claims, JwtService};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::{app_state::AppState, extract::ValidatedJson};
#[derive(Deserialize)]
struct LoginReq {
    username: String,
}

// ===== Middleware de autenticação (definido aqui no main) =====
async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> impl axum::response::IntoResponse {
    use axum::http::{StatusCode, header::AUTHORIZATION};

    // 1) pegar header Authorization
    let Some(h) = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    else {
        return (StatusCode::UNAUTHORIZED, "missing Authorization").into_response();
    };
    // 2) extrair token Bearer
    let Some(token) = h.strip_prefix("Bearer ").map(str::trim) else {
        return (StatusCode::UNAUTHORIZED, "expected Bearer token").into_response();
    };
    // 3) validar com JwtService
    let Ok(claims) = state.jwt.verify(token) else {
        return (StatusCode::UNAUTHORIZED, "invalid or expired").into_response();
    };
    // 4) injetar claims para o handler protegido
    req.extensions_mut().insert::<Claims>(claims);

    next.run(req).await
}

// ===== ROTA PÚBLICA: gera token =====
async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginReq>,
) -> Json<serde_json::Value> {
    // id fictício só para exemplo
    let token = state
        .jwt
        .generate(1u32, &payload.username)
        .expect("falha ao gerar token");
    Json(serde_json::json!({ "token": token }))
}

// ===== ROTA PROTEGIDA: lê claims injetadas pelo middleware =====
async fn me(req: Request) -> Json<serde_json::Value> {
    let claims = req.extensions().get::<Claims>().cloned();
    Json(serde_json::json!({ "claims": claims }))
}

// ===== HEALTH (opcional) =====
async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    // carregar .env e secret
    let _ = dotenvy::dotenv();
    let secret = std::env::var("JWT_SECRET").expect("defina JWT_SECRET no .env");
    let state = AppState::new(JwtService::new(secret.as_bytes()));

    // rotas públicas
    let public = Router::new()
        .route("/", get(|| async { "Hello from API" }))
        .route("/health", get(health))
        .route("/login", post(login));

    // rotas protegidas
    let protected = Router::new()
        .route("/me", get(me))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth));

    let users = users_routes(state.clone());
    // app final com estado compartilhado
    let app = Router::new()
        .merge(public)
        .merge(protected)
        .merge(users)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
