use rocket::{get, http::CookieJar, routes, Route, State};

use crate::{
    api::{
        auth::{
            cookie::{create_auth_cookie, create_expired_cookie},
            google_verification::AuthenticatedGoogleMail,
        },
        helpers::{ArgentApiResult, NewData, OkData},
    },
    config::AuthenticationConfig,
    data::users::{models::User, store::UsersStore},
    error::SimpleMessage,
};

#[get("/login")]
async fn login(
    email: AuthenticatedGoogleMail,
    mut users_store: UsersStore,
    cookies: &CookieJar<'_>,
    auth_config: &State<AuthenticationConfig>,
) -> ArgentApiResult<User> {
    let user = users_store.get_user_for_email(&email.0).await?;
    let auth_cookie = create_auth_cookie(&auth_config, &user);
    cookies.add(auth_cookie);
    ArgentApiResult::new(user)
}

#[get("/logout")]
async fn logout(
    cookies: &CookieJar<'_>,
    auth_config: &State<AuthenticationConfig>,
) -> ArgentApiResult<SimpleMessage> {
    let auth_cookie = create_expired_cookie(&auth_config);
    cookies.add(auth_cookie);
    ArgentApiResult::new_ok()
}

pub fn routes() -> Vec<Route> {
    routes![login, logout]
}
