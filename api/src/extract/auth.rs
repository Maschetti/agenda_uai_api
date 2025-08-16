// auth.rs
use axum::{
    extract::{FromRef, FromRequest, Request},
    http::{header::AUTHORIZATION, StatusCode},
};
use serde_json::Value;

use crate::app_state::AppState;
use crate::response::{ApiError, ApiResponse};
use infra::Claims;

pub struct Auth(pub Claims);

// helper p/ erro padronizado
fn make_err(status: StatusCode, code: &'static str, message: impl Into<String>) -> ApiResponse<Value> {
    ApiResponse {
        status_code: status.as_u16(),
        success: false,
        data: None,
        errors: Some(vec![ApiError {
            code,
            message: message.into(),
        }]),
    }
}

impl<S> FromRequest<S> for Auth
where
    S: Send + Sync,
    AppState: FromRef<S>, // permite extrair AppState de S
{
    type Rejection = ApiResponse<Value>;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // 1) Header Authorization
        let auth = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| make_err(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "missing Authorization header"))?;

        // 2) Bearer <token>
        let token = auth
            .strip_prefix("Bearer ")
            .ok_or_else(|| make_err(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "expected Bearer token"))?;

        // 3) Verificar token
        let claims = app_state
            .jwt
            .verify(token)
            .map_err(|_| make_err(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "invalid or expired token"))?;

        Ok(Auth(claims))
    }
}
