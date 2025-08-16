use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    extract::{Auth, ValidatedJson},
    response::{ApiError, ApiResponse},
};

use core_types::{Cpf, Email, Name, Password, PhoneNumber, Status, Username};
use domain::user::User;

/// ====== POST /auth/login (gera token p/ teste) ======
#[derive(Deserialize)]
struct LoginRequest {
    // Mantive String para simplificar a geração do token
    username: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

async fn login(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> ApiResponse<TokenResponse> {
    let token = match state.jwt.generate(1, &payload.username) {
        Ok(t) => t,
        Err(_) => {
            return ApiResponse {
                status_code: 500,
                success: false,
                data: None,
                errors: Some(vec![ApiError {
                    code: "101",
                    message: "could not generate token".into(),
                }]),
            };
        }
    };

    ApiResponse {
        status_code: 200,
        success: true,
        data: Some(TokenResponse { token }),
        errors: None,
    }
}

/// ====== GET /users/me (protegida) ======
#[derive(Serialize)]
struct MeResponse {
    id: u32,
    username: String,
}

async fn me(Auth(claims): Auth) -> ApiResponse<MeResponse> {
    ApiResponse {
        status_code: 200,
        success: true,
        data: Some(MeResponse {
            id: claims.sub,
            username: claims.username,
        }),
        errors: None,
    }
}

/// ====== POST /users (sem auth, exemplo com ValidatedJson) ======
#[derive(Deserialize)]
struct CreateUserRequest {
    username: Username,
    name: Name,
    cpf: Cpf,
    email: Email,
    password: Password,
    phone_number: PhoneNumber,
    status: Option<Status>,
}

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> ApiResponse<User> {
    // fake persistence
    let user = User {
        id: 1,
        username: payload.username,
        name: payload.name,
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

/// ====== Router ======
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login)) // gera token
        .route("/users/me", get(me)) // exige Authorization: Bearer <token>
        .route("/users", post(create_user)) // sem auth, usa ValidatedJson
}
