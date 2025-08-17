use axum::{Router, extract::State, middleware, routing::post};

use crate::{
    app_state::AppState,
    extract::ValidatedJson,
    request::users::{CreateUserRequest, GetUserRequest},
    response::{
        response::{ApiError, ApiResponse},
        users::CreateUserResponse,
    },
    middlewares::auth::auth_middleware
};

use domain::user::User;

async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> ApiResponse<CreateUserResponse> {
    let token = match state.jwt.generate(1, &payload.username) {
        Ok(t) => t,
        Err(_e) => {
            return ApiResponse {
                status_code: 500,
                success: false,
                data: None,
                errors: Some(vec![ApiError {
                    code: "TOKEN_GENERATION_FAILED",
                    message: "could not generate token".to_string(),
                }]),
            };
        }
    };

    ApiResponse {
        status_code: 201,
        success: true,
        data: Some(CreateUserResponse { token }),
        errors: None,
    }
}

async fn get_user(ValidatedJson(payload): ValidatedJson<GetUserRequest>) -> ApiResponse<User> {
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
        token: "".to_string(),
    };

    ApiResponse {
        status_code: 201,
        success: true,
        data: Some(user),
        errors: None,
    }
}

/// ====== Router ======
pub fn routes(state: AppState) -> Router<AppState> {
    let public = Router::new().route("/users", post(create_user));

    let protected = Router::new()
        .route("/get-user-auth", post(get_user))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    public.merge(protected).with_state(state)
}
