use std::sync::Arc;

use infra::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub jwt: Arc<JwtService>,
}

impl AppState {
    pub fn new(jwt: JwtService) -> Self {
        Self { jwt: Arc::new(jwt) }
    }
}
