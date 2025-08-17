use std::sync::Arc;

use infra::JwtService;
use infra::PasswordHasher;

#[derive(Clone)]
pub struct AppState {
    pub jwt: Arc<JwtService>,
    pub password_hasher: Arc<PasswordHasher>,
    pub users: Arc<Vec<User>>
}

impl AppState {
    pub fn new(jwt: JwtService, password_hasher: PasswordHasher) -> Self {
        Self {
            jwt: Arc::new(jwt),
            password_hasher: Arc::new(password_hasher),
            users: Arc::new([]),
        }
    }
}
