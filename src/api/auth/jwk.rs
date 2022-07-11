use std::{collections::HashMap, sync::Arc};

use jsonwebtoken::{DecodingKey, Validation};
use rocket::{log::private::error, tokio::sync::RwLock};
use serde::Deserialize;

use crate::error::ArgentError;

const CERT_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

#[derive(Deserialize)]
struct GoogleToken {
    email: String,
}

#[derive(Clone, Deserialize)]
struct Jwk {
    pub kid: String,
    pub e: String,
    pub n: String,
}

#[derive(Deserialize)]
struct JwkResponse {
    pub keys: Vec<Jwk>,
}

pub struct Jwks {
    jwk_map: Arc<RwLock<HashMap<String, Jwk>>>,
}

impl Jwks {
    pub async fn new() -> Result<Self, ArgentError> {
        let new = Self {
            jwk_map: Arc::new(RwLock::new(HashMap::new())),
        };
        new.refresh_jwks().await?;
        Ok(new)
    }

    fn validate_with_jwk(jwk: Jwk, token: &str) -> Result<String, ArgentError> {
        let val = Validation::new(jsonwebtoken::Algorithm::RS256);
        let decoded = jsonwebtoken::decode::<GoogleToken>(
            token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?,
            &val,
        )
        .unwrap();
        Ok(decoded.claims.email)
    }

    async fn get_current_keys() -> Result<HashMap<String, Jwk>, ArgentError> {
        Ok(reqwest::get(CERT_URL)
            .await?
            .json::<JwkResponse>()
            .await?
            .keys
            .iter()
            .map(|key| (key.kid.clone(), key.to_owned()))
            .collect::<HashMap<_, _>>())
    }

    async fn refresh_jwks(&self) -> Result<(), ArgentError> {
        let mut jwk_map = self.jwk_map.write().await;
        let current_keys = &Self::get_current_keys().await;
        match current_keys {
            Ok(current_keys) => {
                jwk_map.clone_from(current_keys);
                Ok(())
            }
            Err(err) => {
                drop(jwk_map);
                error!("{}", err);
                Err(ArgentError::server_error())
            }
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<String, ArgentError> {
        // get key id
        let header = jsonwebtoken::decode_header(token)?;
        let kid = header.kid.ok_or_else(|| ArgentError::unauthorized())?;
        // Try using cache
        let jwk = match self.jwk_map.read().await.get(&kid).cloned() {
            Some(jwk) => jwk,
            None => {
                self.refresh_jwks().await?;
                self.jwk_map
                    .read()
                    .await
                    .get(token)
                    .cloned()
                    .ok_or_else(|| ArgentError::unauthorized())?
            }
        };
        Self::validate_with_jwk(jwk, token)
    }
}

unsafe impl Send for Jwks {}
unsafe impl Sync for Jwks {}
