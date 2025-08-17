use argon2::{
    password_hash::{PasswordHasher as _, SaltString}, // <- traz o trait + SaltString
    Algorithm, Argon2, Params, Version,
};
use rand::rngs::OsRng; // <- troque o import do OsRng

#[derive(Clone)]
pub struct PasswordHasher {
    pepper: Vec<u8>,
}

impl PasswordHasher {
    pub fn new(pepper: impl AsRef<[u8]>) -> Self {
        Self { pepper: pepper.as_ref().to_vec() }
    }

    pub fn hasher(&self) -> Argon2<'_> {
        Argon2::new_with_secret(
            &self.pepper,
            Algorithm::Argon2id,
            Version::V0x13,
            Params::default(),
        )
        .expect("argon2 new_with_secret failed: invalid params/secret")
    }

    pub fn hash(&self, plain: &str) -> Result<String, password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let phc  = self.hasher().hash_password(plain.as_bytes(), &salt)?; // trait no escopo -> ok
        Ok(phc.to_string())
    }
}

impl From<password_hash::Error> for ApiResponse<serde_json::Value> {
    fn from(_: password_hash::Error) -> Self {
        ApiResponse::err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "PASSWORD_HASH_FAILED",
            "something went wrong with the password",
        )
    }
}
