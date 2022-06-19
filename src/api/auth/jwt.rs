use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::time::{ext::NumericalDuration, OffsetDateTime};
use serde::{Deserialize, Serialize};

use crate::{config::AuthenticationConfig, data::users::models::User, error::ArgentError};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: i64,
    user: User,
}

impl Claims {
    fn new(user: User) -> Self {
        Self {
            exp: OffsetDateTime::now_utc()
                .saturating_add(30.minutes())
                .unix_timestamp(),
            user,
        }
    }
}

pub fn generate_token(user: &User, auth_config: &AuthenticationConfig) -> String {
    let claims = Claims::new(user.clone());
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config.jwt_key.as_bytes()),
    )
    .expect("Failed making token")
}

pub fn decode_token(token: &str, auth_config: &AuthenticationConfig) -> Result<User, ArgentError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&auth_config.jwt_key.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| {
        println!("{}", err);
        ArgentError::unauthorized()
    })
    .map(|data| data.claims.user)
}
