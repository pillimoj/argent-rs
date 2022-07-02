use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::AuthenticationConfig, data::users::models::User, error::ArgentError};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user: User,
}

impl Claims {
    fn new(user: User, exp: usize) -> Self {
        Self { exp, user }
    }
}

pub fn generate_token(user: &User, auth_config: &AuthenticationConfig, exp: usize) -> String {
    let claims = Claims::new(user.clone(), exp);
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config.jwt_key.as_bytes()),
    )
    .expect("Failed making token")
}

pub fn decode_token(token: &str, auth_config: &AuthenticationConfig) -> Result<User, ArgentError> {
    let user = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&auth_config.jwt_key.as_bytes()),
        &Validation::default(),
    )?
    .claims
    .user;
    Ok(user)
}
