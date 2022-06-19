pub type BoxResult<T> = Result<T, Box<dyn Error>>;

use std::error::Error;
use std::fmt;

use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;

#[derive(Debug)]
pub struct ArgentError {
    pub msg: String,
    pub status: Status,
}

#[derive(Serialize, Debug)]
pub struct SimpleMessage {
    pub msg: String,
}
impl SimpleMessage {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
    pub fn ok() -> Self {
        Self {
            msg: String::from("OK"),
        }
    }
}

impl ArgentError {
    pub fn new(msg: &str, status: Status) -> Self {
        Self {
            msg: msg.to_string(),
            status,
        }
    }

    pub fn not_found() -> Self {
        Self::new("Not found", Status::NotFound)
    }

    pub fn bad_request() -> Self {
        Self::new("Bad Request", Status::BadRequest)
    }

    pub fn forbidden() -> Self {
        Self::new("Forbidden", Status::Forbidden)
    }

    pub fn server_error() -> Self {
        Self::new("Internal Server Error", Status::NotFound)
    }

    pub fn not_found_msg(msg: &str) -> Self {
        Self::new(msg, Status::NotFound)
    }

    pub fn bad_request_msg(msg: &str) -> Self {
        Self::new(msg, Status::BadRequest)
    }

    pub fn forbidden_msg(msg: &str) -> Self {
        Self::new(msg, Status::Forbidden)
    }

    pub fn server_error_msg(msg: &str) -> Self {
        Self::new(msg, Status::NotFound)
    }

    pub fn unauthorized() -> Self {
        Self::new("Unauthorized", Status::Unauthorized)
    }

    pub fn unauthorized_msg(msg: &str) -> Self {
        Self::new(msg, Status::Unauthorized)
    }

    pub fn from_error(err: &dyn Error) -> Self {
        println!("{}", err);
        Self::server_error()
    }
}

impl fmt::Display for ArgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.status, self.msg)
    }
}

impl Error for ArgentError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<(Status, ArgentError)> for ArgentError {
    fn from((_, error): (Status, ArgentError)) -> Self {
        error
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ArgentError {
    fn respond_to(self, req: &Request) -> response::Result<'o> {
        response::Response::build_from(Json(SimpleMessage::new(self.msg)).respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
