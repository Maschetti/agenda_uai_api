// api/src/response/mod.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status_code: u16,
    pub success: bool,
    pub data: Option<T>,
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: String,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            "{\"status_code\":500,\"success\":false,\"data\":null,\
              \"errors\":[{\"code\":\"SERIALIZATION_ERROR\",\
              \"message\":\"Failed to serialize response\"}]}".to_string()
        });

        (status, [("Content-Type", "application/json")], body).into_response()
    }
}
