use rocket::http::{Cookie, SameSite};
use rocket::time::OffsetDateTime;

use crate::{config::AuthenticationConfig, data::users::models::User};

use super::jwt::generate_token;

pub fn create_auth_cookie(config: &AuthenticationConfig, user: &User) -> Cookie<'static> {
    println!("Adding cookie {}", &config.cookie_name);
    Cookie::build(
        config.cookie_name.to_string(),
        generate_token(&user, config),
    )
    .http_only(true)
    .secure(config.secure_cookie)
    .same_site(if config.secure_cookie {
        SameSite::None
    } else {
        SameSite::Strict
    })
    .path("/api/v1")
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
