use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::Json,
    Request,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct SimpleMessage {
    pub msg: String,
}
impl SimpleMessage {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
    pub fn ok() -> Self {
        Self {
            msg: String::from("OK"),
        }
    }
}

#[derive(Error, Debug)]
pub enum ArgentError {
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("{msg}")]
    Api { status: Status, msg: String },
    #[error(transparent)]
    Server(#[from] anyhow::Error),
    #[error(transparent)]
    ExternalServer(#[from] reqwest::Error),
    #[error(transparent)]
    DataBase(#[from] sqlx::Error),
}

impl ArgentError {
    fn for_api(&self) -> (Status, SimpleMessage) {
        (self.status_code(), self.simple_message())
    }

    pub fn status_code(&self) -> Status {
        match self {
            Self::Api { status, .. } => *status,
            Self::Jwt(_) => Status::Unauthorized,
            Self::Server(_) | Self::ExternalServer(_) | Self::DataBase(_) => {
                Status::InternalServerError
            }
        }
    }

    fn simple_message(&self) -> SimpleMessage {
        match self {
            Self::Api { msg, .. } => SimpleMessage::new(msg),
            Self::Jwt(_) => SimpleMessage::new("Unauthorized"),
            Self::Server(_) | Self::ExternalServer(_) | Self::DataBase(_) => {
                SimpleMessage::new("Internal Server Error")
            }
        }
    }

    pub fn new(msg: &str, status: Status) -> Self {
        Self::Api {
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

    pub fn unauthorized() -> Self {
        Self::new("Unauthorized", Status::Unauthorized)
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

    pub fn unauthorized_msg(msg: &str) -> Self {
        Self::new(msg, Status::Unauthorized)
    }
    pub fn server_error_msg(msg: &str) -> Self {
        Self::new(msg, Status::NotFound)
    }

    pub fn from_status(status: Status) -> Self {
        match status.class() {
            rocket::http::StatusClass::ClientError => match status.code {
                400 => Self::bad_request(),
                401 => Self::unauthorized(),
                403 => Self::forbidden(),
                404 => Self::not_found(),
                _ => panic!(
                    "Creating ArgentError from Status {} not implemented",
                    status.code
                ),
            },
            rocket::http::StatusClass::ServerError | rocket::http::StatusClass::Unknown => {
                Self::server_error()
            }
            _ => panic!("Creating error from non error status"),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ArgentError {
    fn respond_to(self, req: &Request) -> response::Result<'o> {
        let (status, msg) = self.for_api();
        response::Response::build_from(Json(msg).respond_to(&req)?)
            .status(status)
            .header(ContentType::JSON)
            .ok()
    }
}
