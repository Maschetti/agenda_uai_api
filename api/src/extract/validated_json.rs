use axum::{
    body,
    extract::{FromRequest, Request},
    http::header::CONTENT_TYPE,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::response::{ApiError, ApiResponse};

pub struct ValidatedJson<T>(pub T);

/// Remove o sufixo " at line N column M"
fn clean_serde_error(err: serde_json::Error) -> String {
    let mut msg = err.to_string();
    if let Some(idx) = msg.find(" at line") {
        msg.truncate(idx);
    }
    msg
}

/// Traduz mensagens comuns do serde_json para português
fn prettify(msg: &str) -> String {
    if let Some(rest) = msg.strip_prefix("missing field `") {
        if let Some(field) = rest.strip_suffix('`') {
            return format!("campo obrigatório: {}", field);
        }
    }

    if let Some(rest) = msg.strip_prefix("unknown field `") {
        if let Some(after) = rest.find("`") {
            let field = &rest[..after];
            return format!("campo desconhecido: {}", field);
        }
    }

    if msg.starts_with("expected") {
        return format!("formato inválido: {}", msg);
    }

    // fallback: mantém como veio
    msg.to_string()
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiResponse<Value>;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        // 1) Checar Content-Type
        let is_json = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.split(';').next().map(str::trim) == Some("application/json"))
            .unwrap_or(false);

        let (_parts, body_stream) = req.into_parts();

        // 2) Ler corpo
        let bytes = body::to_bytes(body_stream, 1024 * 1024)
            .await
            .map_err(|_| ApiResponse {
                status_code: 400,
                success: false,
                data: None,
                errors: Some(vec![ApiError {
                    code: "BAD_REQUEST",
                    message: "não foi possível ler o corpo da requisição".into(),
                }]),
            })?;

        // 3) Rejeitar se não for JSON
        if !is_json {
            return Err(ApiResponse {
                status_code: 415,
                success: false,
                data: None,
                errors: Some(vec![ApiError {
                    code: "UNSUPPORTED_MEDIA_TYPE",
                    message: "o corpo deve ser JSON (Content-Type: application/json)".into(),
                }]),
            });
        }

        // 4) Desserializar com erro tratado
        let value: T = serde_json::from_slice(&bytes).map_err(|e| {
            let raw = clean_serde_error(e);
            let pretty = prettify(&raw);

            ApiResponse {
                status_code: 422,
                success: false,
                data: None,
                errors: Some(vec![ApiError {
                    code: "BAD_REQUEST",
                    message: pretty,
                }]),
            }
        })?;

        Ok(ValidatedJson(value))
    }
}
