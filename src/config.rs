use rocket::serde::json::serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfig {
    pub jwt_key: String,
    pub secure_cookie: bool,
    pub cookie_name: String,
}

impl AuthenticationConfig {
    pub fn from_env() -> Self {
        let as_string = std::env::var("ARGENT_AUTH").unwrap();
        serde_json::from_str(&as_string).unwrap()
    }
}

unsafe impl Send for AuthenticationConfig {}
unsafe impl Sync for AuthenticationConfig {}
