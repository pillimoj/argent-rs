pub mod data;

pub mod api {
    pub mod auth {
        pub mod cookie;
        pub mod google_verification;
        pub mod jwk;
        pub mod jwt;
        pub mod user_guard;
    }
    pub mod helpers;
    pub mod v1;
}
pub mod config;
pub mod cors;
pub mod error;

use std::fs;

use crate::{api::v1::ApiV1Routes, data::ArgentDB};
use api::auth::jwk::Jwks;
use config::AuthenticationConfig;
use cors::CORS;
use error::SimpleMessage;
use rocket::{get, launch, routes, serde::json::Json};
use rocket_db_pools::Database;

//#[macro_use]
extern crate rocket;

pub const ARGENT_DEBUG: bool = cfg!(debug_assertions);

#[get("/health-check")]
async fn health_check() -> Json<SimpleMessage> {
    Json(SimpleMessage::ok())
}

#[get("/ping")]
async fn ping() -> Json<SimpleMessage> {
    Json(SimpleMessage::ok())
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(
            Jwks::new()
                .await
                .expect("Could not start google Jwt verifier"),
        )
        .manage(AuthenticationConfig::from_env())
        .attach(ArgentDB::init())
        .attach(CORS::init())
        .mount("/api/v1", ApiV1Routes::get())
        .mount("/", routes![ping, health_check])
}
