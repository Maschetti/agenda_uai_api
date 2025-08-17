use axum::{Router, extract::State, middleware, routing::post};

use crate::{
    app_state::AppState,
    extract::ValidatedJson,
    middlewares::auth::auth_middleware,
    request::users::{CreateUserRequest, GetUserByIdRequest, GetUserRequest},
    response::{
        response::{ApiError, ApiResponse},
        users::{CreateUserResponse, GetUserByIdResponse},
    },
};

use domain::user::User;

async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> ApiResponse<CreateUserResponse> {
    let token = state.jwt.generate(1, &payload.email.as_str())?;

    let token = state.password_hasher.hash(&payload.password.as_str())?;

    ApiResponse {
        status_code: 201,
        success: true,
        data: Some(CreateUserResponse { token }),
        errors: None,
    }
}

async fn get_user_by_id(ValidatedJson(payload): ValidatedJson<GetUserByIdRequest>) -> ApiResponse<GetUserByIdResponse> {
    
    // fake persistence
    let user = User {
        id: 1,
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
        .route("/users", get(get_user))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public.merge(protected).with_state(state)
}
