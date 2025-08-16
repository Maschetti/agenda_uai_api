use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: u32,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Clone)]
pub struct JwtService {
    enc: EncodingKey,
    alg: Algorithm,
    dec: DecodingKey,
    ttl: Duration,
}

impl JwtService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            enc: EncodingKey::from_secret(secret),
            dec: DecodingKey::from_secret(secret),
            alg: Algorithm::HS256,
            ttl: Duration::minutes(30),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn generate(&self, id: u32, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + self.ttl;

        let claims = Claims {
            sub: id,
            username: username.to_owned(),
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        };

        let header = Header::new(self.alg);
        encode(&header, &claims, &self.enc)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(self.alg);
        // validação de exp já é padrão, mas deixo explícito:
        validation.validate_exp = true;

        let data = decode::<Claims>(token, &self.dec, &validation)?;
        Ok(data.claims)
    }
}
