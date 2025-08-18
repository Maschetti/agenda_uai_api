use std::sync::Arc;
use tokio::sync::Mutex;

use domain::user::User;
use infra::{JwtService, PasswordHasher};

#[derive(Clone)]
pub struct AppState {
    pub jwt: Arc<JwtService>,
    pub password_hasher: Arc<PasswordHasher>,
    pub users: Arc<Mutex<Vec<User>>>,
}

impl AppState {
    pub fn new(jwt: JwtService, password_hasher: PasswordHasher) -> Self {
        Self {
            jwt: Arc::new(jwt),
            password_hasher: Arc::new(password_hasher),
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
