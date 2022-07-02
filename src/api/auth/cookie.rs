use rocket::http::{Cookie, SameSite};
use rocket::time::{ext::NumericalDuration, OffsetDateTime};

use crate::{config::AuthenticationConfig, data::users::models::User};

use super::jwt::generate_token;

pub fn create_auth_cookie(config: &AuthenticationConfig, user: &User) -> Cookie<'static> {
    let duration = 30.minutes();
    let expiry_date = OffsetDateTime::now_utc().saturating_add(duration);
    // Since timestamps will always be positive this will not fail
    // Positive i64 fits in usize (32/64 bit unsigned)
    let exp: usize = expiry_date.unix_timestamp().try_into().unwrap();
    Cookie::build(
        config.cookie_name.to_string(),
        generate_token(&user, config, exp),
    )
    .http_only(true)
    .secure(config.secure_cookie)
    .same_site(if config.secure_cookie {
        SameSite::None
    } else {
        SameSite::Strict
    })
    .path("/api/v1")
    .expires(expiry_date)
    .finish()
}

pub fn create_expired_cookie(config: &AuthenticationConfig) -> Cookie<'static> {
    Cookie::build(config.cookie_name.to_string(), "")
        .http_only(true)
        .secure(config.secure_cookie)
        .same_site(if config.secure_cookie {
            SameSite::None
        } else {
            SameSite::Strict
        })
        .path("/api/v1")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .finish()
}
