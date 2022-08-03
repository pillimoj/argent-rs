use rocket::{
    http::Status,
    serde::{Deserialize, Serialize},
};
use sqlx::{
    postgres::PgRow,
    types::time::{OffsetDateTime, PrimitiveDateTime},
    FromRow, Row, Type,
};
use uuid::Uuid;

use crate::{api::helpers::parse_uuid, error::ArgentError};

#[derive(Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AccessType {
    Owner,
    Editor,
    None,
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
        let checklist = parse_uuid(&self.checklist, Status::BadRequest)?;
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
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: Uuid,
    pub title: String,
    pub checklist: Uuid,
    pub done: bool,
    pub created_at: i64,
}
impl ChecklistItem {
    pub fn from_row(row: &PgRow) -> Result<ChecklistItem, ArgentError> {
        Ok(ChecklistItem {
            id: row.try_get::<Uuid, _>("id")?,
            title: row.try_get::<String, _>("title")?,
            checklist: row.try_get::<Uuid, _>("checklist")?,
            done: row.try_get::<bool, _>("done")?,
            created_at: row
                .try_get::<PrimitiveDateTime, _>("created_at")?
                .assume_utc()
                .unix_timestamp(),
        })
    }
    pub fn created_at_primitive_datetime(&self) -> PrimitiveDateTime {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(self.created_at);
        PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time())
    }
}

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserAccess {
    id: Uuid,
    name: String,
    pub access_type: AccessType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareRequest {
    pub user_id: String,
    pub access_type: AccessType,
}
