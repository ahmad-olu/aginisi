use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // aud: String,     // Audience // ! optional
    pub exp: usize,  // Expiration time (as UTC timestamp)
    pub iat: usize,  // Issued at (as UTC timestamp) // ! optional
    pub iss: String, // Issuer // ! optional
    pub nbf: usize,  // Not Before (as UTC timestamp) // ! optional
    pub sub: i64,    // Subject (whom the token refers to) // ! optional
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
