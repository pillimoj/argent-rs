use jwks_client::keyset::KeyStore;
use rocket::http::Status;

use crate::error::ArgentError;

pub struct JWKSStore {
    store: KeyStore,
}

impl JWKSStore {
    pub async fn from_url(url: &str) -> Result<JWKSStore, ArgentError> {
        let store = KeyStore::new_from(url.to_string()).await.unwrap();
        Ok(JWKSStore { store })
    }

    pub async fn google() -> Result<JWKSStore, ArgentError> {
        Self::from_url("https://www.googleapis.com/oauth2/v3/certs").await
    }

    pub async fn verify_token(&self, token: &str) -> Result<String, ArgentError> {
        let jwt = self.store.verify(token).map_err(Self::conv_client_err)?;
        let email = jwt
            .payload()
            .get_str("email")
            .map(str::to_string)
            .ok_or_else(|| ArgentError::new("token missing email", Status::Unauthorized));
        email
    }

    fn conv_client_err(cr: jwks_client::error::Error) -> ArgentError {
        ArgentError::new(
            &format!("jwks_client - {:?} - {}", cr.typ, cr.msg),
            Status::Unauthorized,
        )
    }
}

unsafe impl Send for JWKSStore {}
unsafe impl Sync for JWKSStore {}
