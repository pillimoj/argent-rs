use std::collections::HashSet;

use rocket::{
    fairing::{Fairing, Info, Kind},
    route::Outcome,
    serde::json::Json,
    Route,
};
use rocket::{
    http::{Header, Method},
    route::Handler,
};
use rocket::{Request, Response};

pub struct CORS {
    debug: bool,
}

#[derive(Clone)]
pub struct OptionsHandler;

#[rocket::async_trait]
impl Handler for OptionsHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, _data: rocket::data::Data<'r>) -> Outcome<'r> {
        Outcome::from(req, Json("ok"))
    }
}

impl CORS {
    pub fn init() -> Self {
        Self {
            debug: std::env::var("ARGENT_DEBUG").unwrap_or(String::from("false")) == "true",
        }
    }
    pub fn add_options_method(routes: Vec<Route>) -> Vec<Route> {
        let unique_routes: HashSet<&str> = routes.iter().map(|r| r.uri.as_str()).collect();

        let option_routes = unique_routes
            .iter()
            .map(|uri| Route::new(Method::Options, uri, OptionsHandler))
            .collect();
        [routes, option_routes].concat()
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let origin = if self.debug {
            "localhost:8008"
        } else {
            "https://argent.grimsborn.com"
        };
        response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "PATCH, OPTIONS, PUT, DELETE",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, X-Forwarded-Proto, X-Request-ID, X-Requested-With",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
