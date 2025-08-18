use axum::{
    extract::{Path, State},
    middleware,
    routing::{get, post},
    Router,
};
use axum::http::StatusCode;

use crate::{
    app_state::AppState,
    extract::ValidatedJson,
    middlewares::auth::auth_middleware,
    request::users::{CreateUserRequest, GetUserByEmailRequest, GetUserByIdRequest},
    response::{
        response::{ApiError, ApiResponse},
        users::{CreateUserResponse, GetUserByEmailResponse, GetUserByIdResponse},
    },
    try_or,
};
use domain::user::User;

// POST /users
async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> ApiResponse<CreateUserResponse> {
    let token = try_or!(
        state.jwt.generate(1, payload.email.as_str()),
        ApiResponse::err_token()
    );

    let hashed_password = try_or!(
        state.password_hasher.hash(payload.password.as_str()),
        ApiResponse::err_hash()
    );

    let mut users = state.users.lock().await;
    let new_id = users.len() as u32 + 1;

    users.push(User {
        id: new_id,
        name: payload.name,
        cpf: payload.cpf,
        email: payload.email,
        password_hash: hashed_password,
        phone_number: payload.phone_number,
        status: payload.status,
        token: token.clone(),
    });

    ApiResponse {
        status_code: 201,
        success: true,
        data: Some(CreateUserResponse { token }),
        errors: None,
    }
}

// GET /users/{id}
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(GetUserByIdRequest { id }): Path<GetUserByIdRequest>,
) -> ApiResponse<GetUserByIdResponse> {
    let users = state.users.lock().await;

    if let Some(u) = users.iter().find(|u| u.id == id) {
        // monte o DTO direto aqui
        let resp = GetUserByIdResponse {
            id: u.id,
            name: u.name.clone(),
            cpf: u.cpf.clone(),
            phone_number: u.phone_number.clone(),
            status: u.status.clone(),
            token: u.token.clone()
        };

        ApiResponse {
            status_code: 200,
            success: true,
            data: Some(resp),
            errors: None,
        }
    } else {
        ApiResponse {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError {
                code: "NOT_FOUND",
                message: format!("Usuário {id} não encontrado"),
            }]),
        }
    }
}

// GET /users/{id}
async fn get_user_by_email(
    State(state): State<AppState>,
    Path(GetUserByEmailRequest { email }): Path<GetUserByEmailRequest>,
) -> ApiResponse<GetUserByEmailResponse> {
    let users = state.users.lock().await;

    if let Some(u) = users.iter().find(|u| u.email == email) {
        // monte o DTO direto aqui
        let resp = GetUserByEmailResponse {
            id: u.id,
            name: u.name.clone(),
            cpf: u.cpf.clone(),
            phone_number: u.phone_number.clone(),
            status: u.status.clone(),
            token: u.token.clone()
        };

        ApiResponse {
            status_code: 200,
            success: true,
            data: Some(resp),
            errors: None,
        }
    } else {
        ApiResponse {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            success: false,
            data: None,
            errors: Some(vec![ApiError {
                code: "NOT_FOUND",
                message: format!("Usuário não encontrado"),
            }]),
        }
    }
}

// GET /users
async fn get_all_users(State(state): State<AppState>) -> ApiResponse<Vec<User>> {
    let users = state.users.lock().await;

    ApiResponse {
        status_code: 200,
        success: true,
        data: Some(users.clone()),
        errors: None,
    }
}

/// ====== Router ======
pub fn routes(state: AppState) -> Router<AppState> {
    let public = Router::new()
        .route("/create", post(create_user));

    let protected = Router::new()
        .route("/get-by-id/{id}", get(get_user_by_id)) // <- chave fechada e sintaxe 0.8
        .route("/get-by-email/{email}", get(get_user_by_email))
        .route("/get-all", get(get_all_users))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/users", public.merge(protected))
        .with_state(state)
}
