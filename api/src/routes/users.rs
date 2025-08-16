use axum::{routing::post, Router};
use domain::user::User;
use serde::{Deserialize};
use core_types::{Cpf, Email, Password, PhoneNumber, Status};

use crate::{
    app_state::AppState,    // <- seu ValidatedJson
    ApiResponse,
};
use crate::extract::{ValidatedJson, Auth};

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    cpf: Cpf,
    email: Email,
    password: Password,
    phone_number: PhoneNumber,
    status: Status,
}

// Handler: POST /users
async fn create_user(ValidatedJson(payload): ValidatedJson<CreateUserRequest>) -> ApiResponse<User> {

    // fake persistence for now
    let user = User {
        id: 1,
        username: payload.username,
        cpf: payload.cpf,
        email: payload.email,
        password: payload.password,
        phone_number: payload.phone_number,
        status: payload.status,
    };

    ApiResponse {
        status_code: 201,
        success: true,
        data: Some(user),
        errors: None,
    }
}

// Public function that returns a Router with user routes
pub fn routes() -> Router {
    Router::new().route("/users", post(create_user))
}
