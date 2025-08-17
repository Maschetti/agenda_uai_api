use axum::{extract::{State, Request}, middleware::Next, response::IntoResponse};
use infra::Claims;

use crate::app_state::AppState;

pub async fn auth_middleware(
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