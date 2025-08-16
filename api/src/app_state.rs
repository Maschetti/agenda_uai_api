// app_state.rs
use std::sync::Arc;

use infra::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub jwt: Arc<JwtService>,
}
