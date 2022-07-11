use rocket::{
    http::{hyper::header::AUTHORIZATION, Status},
    log::private::warn,
    outcome::{try_outcome, IntoOutcome, Outcome},
    request::FromRequest,
    Request,
};

use crate::error::ArgentError;

use super::jwk::Jwks;

struct GoogleToken<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GoogleToken<'r> {
    type Error = ArgentError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<GoogleToken<'r>, (Status, Self::Error), ()> {
        let token = request
            .headers()
            .get(AUTHORIZATION.as_str())
            .next()
            .map(|string| string.trim_start_matches("Bearer "))
            .map(GoogleToken)
            .into_outcome((
                Status::BadRequest,
                ArgentError::new(
                    "Missing google authorization bearer token",
                    Status::BadRequest,
                ),
            ));
        token
    }
}

pub struct AuthenticatedGoogleMail(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedGoogleMail {
    type Error = ArgentError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<AuthenticatedGoogleMail, (Status, Self::Error), ()> {
        let token = request.guard::<GoogleToken>().await;
        let token = try_outcome!(token);
        let jwks = request
            .rocket()
            .state::<Jwks>()
            .expect("Could not access JWKSStore");
        match jwks.validate_token(token.0).await {
            Ok(verified_email) => Outcome::Success(Self(verified_email)),
            Err(error) => {
                warn!("Could not verify token - {}", error);
                Outcome::Failure((Status::Unauthorized, ArgentError::unauthorized()))
            }
        }
    }
}
