use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub token: String,
}
