use rocket::{
    http::{CookieJar, Status},
    outcome::Outcome,
    request::FromRequest,
    Request,
};

use crate::{config::AuthenticationConfig, data::users::models::User, error::ArgentError};

use super::jwt::decode_token;

fn get_user_from_cookie(
    cookies: &CookieJar,
    auth_config: &AuthenticationConfig,
) -> Result<User, ArgentError> {
    let token = cookies
        .get(&auth_config.cookie_name)
        .map(|cookie| cookie.value())
        .ok_or(ArgentError::unauthorized_msg("No authentication cookie"))?;
    decode_token(token, auth_config)
}

pub struct AuthenticatedUser(User);
impl AuthenticatedUser {
    pub fn get(self) -> User {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ArgentError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<AuthenticatedUser, (Status, Self::Error), ()> {
        let cookies = request.guard::<&CookieJar>().await.unwrap();
        let auth_config = request.rocket().state::<AuthenticationConfig>().unwrap();
        let user = get_user_from_cookie(cookies, &auth_config);

        match user {
            Ok(user) => Outcome::Success(AuthenticatedUser(user)),
            Err(error) => Outcome::Failure((error.status, error)),
        }
    }
}
