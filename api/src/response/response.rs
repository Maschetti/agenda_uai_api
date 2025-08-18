use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;
use serde_json::Value;

// ===== ApiResponse =====

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status_code: u16,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn err_token() -> Self {
        ApiResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError {
                code: "JWT_ERROR",
                message: "falha ao gerar token".into(),
            }]),
        }
    }

    pub fn err_hash() -> Self {
        ApiResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError {
                code: "HASH_ERROR",
                message: "falha ao gerar hash da senha".into(),
            }]),
        }
    }

    pub fn err_not_found(msg: impl Into<String>) -> Self {
        ApiResponse {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError {
                code: "NOT_FOUND",
                message: msg.into(),
            }]),
        }
    }
}

// Erros ficam no impl específico (sem genérico)
impl ApiResponse<Value> {
    pub fn err(status: StatusCode, code: &'static str, msg: impl Into<String>) -> Self {
        ApiResponse {
            status_code: status.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError { code, message: msg.into() }]),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, axum::Json(self)).into_response()
    }
}
