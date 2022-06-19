use rocket::{
    http::{hyper::header::AUTHORIZATION, Status},
    outcome::{try_outcome, IntoOutcome, Outcome},
    request::FromRequest,
    Request,
};

use crate::{api::auth::jwk::JWKSStore, error::ArgentError};

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
        let store = request
            .rocket()
            .state::<JWKSStore>()
            .expect("Could not access JWKSStore");
        match store.verify_token(token.0).await {
            Ok(verified_token) => Outcome::Success(Self(verified_token)),
            Err(error) => {
                println!("Could not verify token - {}", error);
                Outcome::Failure((
                    Status::Unauthorized,
                    ArgentError::new("Missing authorization bearer token", Status::BadRequest),
                ))
            }
        }
    }
}
