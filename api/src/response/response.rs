// api/src/response/mod.rs
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status_code: u16,
    pub success: bool,

    // omit from JSON if None (keeps responses cleaner)
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
    // Helpers for success cases
    pub fn ok(data: T) -> Self {
        Self { status_code: StatusCode::OK.as_u16(), success: true, data: Some(data), errors: None }
    }

    pub fn created(data: T) -> Self {
        Self { status_code: StatusCode::CREATED.as_u16(), success: true, data: Some(data), errors: None }
    }

    // Helper for error responses (returns ApiResponse<serde_json::Value>)
    pub fn err(status: StatusCode, code: &'static str, msg: impl Into<String>) -> ApiResponse<serde_json::Value> {
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

        // axum sets Content-Type for Json automatically
        (status, Json(self)).into_response()
    }
}
