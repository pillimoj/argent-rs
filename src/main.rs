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
pub mod debugging;
pub mod error;

use crate::{api::v1::ApiV1Routes, data::ArgentDB};
use api::auth::jwk::Jwks;
use config::AuthenticationConfig;
use cors::CORS;
use data::run_migrations;
use debugging::{init_dev_admin, load_debug_env};
use error::SimpleMessage;
use rocket::{fairing::AdHoc, get, launch, routes, serde::json::Json};
use rocket_db_pools::Database;

//#[macro_use]
extern crate rocket;

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
    load_debug_env();
    rocket::build()
        .attach(ArgentDB::init())
        .attach(AdHoc::try_on_ignite("Migrate database", run_migrations))
        .attach(AdHoc::on_ignite(
            "Debug: init admin account",
            init_dev_admin,
        ))
        .manage(
            Jwks::new()
                .await
                .expect("Could not start google Jwt verifier"),
        )
        .manage(AuthenticationConfig::from_env())
        .attach(CORS::init())
        .mount("/api/v1", ApiV1Routes::get())
        .mount("/", routes![ping, health_check])
}
