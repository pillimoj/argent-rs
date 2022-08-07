use rocket::{http::Status, response::Responder, serde::json::Json};
use serde::Serialize;
use uuid::Uuid;

use crate::error::{ArgentError, SimpleMessage};

pub type ArgentApiResult<T> = Result<Data<T>, ArgentError>;
pub trait NewData<T: Serialize> {
    fn new(data: T) -> Self;
}
impl<T: Serialize> NewData<T> for ArgentApiResult<T> {
    fn new(data: T) -> Self {
        Ok(Data::D(data))
    }
}

pub trait OkData {
    fn new_ok() -> Self;
}
impl OkData for ArgentApiResult<SimpleMessage> {
    fn new_ok() -> Self {
        ArgentApiResult::new(SimpleMessage::ok())
    }
}

pub type ArgentResult<T> = Result<T, ArgentError>;
pub trait ApiResultFrom<T: Serialize> {
    fn api(self) -> ArgentApiResult<T>;
}
impl<T: Serialize> ApiResultFrom<T> for ArgentResult<T> {
    fn api(self) -> ArgentApiResult<T> {
        self.map(Data::D)
    }
}
pub enum Data<T: Serialize> {
    D(T),
    Empty,
}

pub fn parse_uuid(string_value: &str, invalid_result: Status) -> ArgentResult<Uuid> {
    Uuid::parse_str(string_value).map_err(|_| ArgentError::from_status(invalid_result))
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for Data<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Data::Empty => panic!("LOLO"), //Json(SimpleMessage::ok()).respond_to(request),
            Data::D(data) => Json(data).respond_to(request),
        }
    }
}

impl<T: Serialize> From<T> for Data<T> {
    fn from(data: T) -> Self {
        Data::D(data)
    }
}

pub fn convert_uuid(rocket_uuid: &rocket::serde::uuid::Uuid) -> Uuid {
    let bytes: &[u8; 16] = rocket_uuid.as_bytes();
    Uuid::from_bytes(*bytes)
}
