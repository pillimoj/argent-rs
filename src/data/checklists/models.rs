use rocket::serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    types::time::{OffsetDateTime, PrimitiveDateTime},
    FromRow, Row, Type,
};
use uuid::Uuid;

use crate::error::ArgentError;

#[derive(Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AccessType {
    Owner,
    Editor,
}

#[derive(Deserialize)]
pub struct ChecklistRequest {
    name: String,
}

#[derive(Serialize, FromRow)]
pub struct Checklist {
    pub id: Uuid,
    pub name: String,
}

impl Checklist {
    pub fn from_request(request: ChecklistRequest) -> Checklist {
        Checklist {
            id: Uuid::new_v4(),
            name: request.name,
        }
    }
}

#[derive(Deserialize)]
pub struct ChecklistItemRequest {
    title: String,
    checklist: String,
}

impl ChecklistItemRequest {
    pub fn get(self) -> Result<ChecklistItem, ArgentError> {
        let checklist = Uuid::parse_str(&self.checklist)
            .map_err(|_| ArgentError::bad_request_msg("Invalid ChecklistItem.id"))?;
        Ok(ChecklistItem {
            id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            title: self.title,
            checklist,
            done: false,
        })
    }
}

#[derive(Serialize, FromRow)]
pub struct ChecklistItem {
    pub id: Uuid,
    pub title: String,
    pub checklist: Uuid,
    pub done: bool,
    pub created_at: i64,
}
impl ChecklistItem {
    pub fn from_row(row: &PgRow) -> ChecklistItem {
        ChecklistItem {
            id: row.try_get::<Uuid, _>("id").unwrap(),
            title: row.try_get::<String, _>("title").unwrap(),
            checklist: row.try_get::<Uuid, _>("checklist").unwrap(),
            done: row.try_get::<bool, _>("done").unwrap(),
            created_at: row
                .try_get::<PrimitiveDateTime, _>("created_at")
                .unwrap()
                .assume_utc()
                .unix_timestamp(),
        }
    }
    pub fn created_at_primitive_datetime(&self) -> PrimitiveDateTime {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(self.created_at);
        PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time())
    }
}

#[derive(Serialize, FromRow)]
pub struct UserAccess {
    id: Uuid,
    name: String,
    pub access_type: AccessType,
}

#[derive(Deserialize)]
pub struct ShareRequest {
    pub user_id: String,
    pub access_type: AccessType,
}
