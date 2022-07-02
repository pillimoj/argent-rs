use std::convert::Infallible;

use rocket::request::FromRequest;
use rocket_db_pools::Connection;
use sqlx::{Acquire, Row};
use uuid::Uuid;

use crate::{
    api::helpers::ArgentResult,
    data::{users::models::User, ArgentDB},
    error::ArgentError,
};

use super::models::{AccessType, Checklist, ChecklistItem, UserAccess};

pub struct ChecklistStore {
    db: Connection<ArgentDB>,
}

impl ChecklistStore {
    pub async fn get_checklist_items(
        &mut self,
        checklist: Uuid,
    ) -> ArgentResult<Vec<ChecklistItem>> {
        sqlx::query(
            "SELECT
                        id,
                        title,
                        done,
                        created_at,
                        checklist
                    FROM checklistitems
                    WHERE checklist = $1",
        )
        .bind(checklist)
        .fetch_all(&mut *self.db)
        .await?
        .iter()
        .map(ChecklistItem::from_row)
        .collect()
    }

    pub async fn get_checklists(&mut self) -> ArgentResult<Vec<Checklist>> {
        let list = sqlx::query_as(
            "SELECT
                    id,
                    name,
                FROM checklists",
        )
        .fetch_all(&mut *self.db)
        .await?;
        Ok(list)
    }

    pub async fn get_checklist_by_id(&mut self, id: Uuid) -> ArgentResult<Checklist> {
        let result = sqlx::query_as(
            "SELECT
                    id,
                    name
                FROM checklists
                WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&mut *self.db)
        .await?;
        match result {
            Some(checklist) => Ok(checklist),
            None => Err(ArgentError::not_found()),
        }
    }

    pub async fn get_checklists_for_user(
        &mut self,
        user: User,
    ) -> Result<Vec<Checklist>, ArgentError> {
        let list = sqlx::query_as(
            "SELECT id, name
                FROM checklists c
                LEFT JOIN checklist_access ca
                ON c.id = ca.checklist
                WHERE ca.argent_user = $1",
        )
        .bind(user.id)
        .fetch_all(&mut *self.db)
        .await?;
        Ok(list)
    }

    pub async fn create_checklist(
        &mut self,
        checklist: Checklist,
        user: User,
    ) -> Result<(), ArgentError> {
        let mut tx = self.db.begin().await?;
        sqlx::query(
            "INSERT INTO checklists (
                    id,
                    name
                )
                VALUES($1,$2)",
        )
        .bind(checklist.id)
        .bind(checklist.name)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO checklist_access (
                checklist,
                argent_user,
                access_type
            )
            VALUES($1,$2,$3)",
        )
        .bind(checklist.id)
        .bind(user.id)
        .bind(AccessType::Owner)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn add_user_access(
        &mut self,
        checklist_id: Uuid,
        user_id: Uuid,
        access_type: AccessType,
    ) -> Result<(), ArgentError> {
        sqlx::query(
            "INSERT INTO checklist_access (
                checklist,
                argent_user,
                access_type
            )
            VALUES($1,$2,$3)",
        )
        .bind(checklist_id)
        .bind(user_id)
        .bind(access_type)
        .execute(&mut *self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_checklist(&mut self, checklist: Uuid) -> Result<(), ArgentError> {
        let mut tx = self.db.begin().await?;

        sqlx::query(
            "DELETE FROM checklistitems
                WHERE checklist = $1",
        )
        .bind(checklist)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "DELETE FROM checklists
                WHERE id = $1",
        )
        .bind(checklist)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn add_item(&mut self, item: ChecklistItem) -> Result<(), ArgentError> {
        sqlx::query(
            "INSERT INTO checklistitems (
                id,
                title,
                done,
                checklist,
                created_at
            )
            VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(&item.id)
        .bind(&item.title)
        .bind(&item.done)
        .bind(&item.checklist)
        .bind(item.created_at_primitive_datetime())
        .execute(&mut *self.db)
        .await?;
        Ok(())
    }

    pub async fn set_item_done(&mut self, item_id: Uuid, done: bool) -> Result<(), ArgentError> {
        sqlx::query(
            " UPDATE checklistitems
            SET done = $1
            WHERE id = $2",
        )
        .bind(done)
        .bind(item_id)
        .execute(&mut *self.db)
        .await?;
        Ok(())
    }

    pub async fn clear_done(&mut self, checklist: Uuid) -> Result<(), ArgentError> {
        sqlx::query(
            "DELETE FROM checklistitems
                WHERE checklist = $1
                AND done",
        )
        .bind(checklist)
        .execute(&mut *self.db)
        .await?;
        Ok(())
    }

    pub async fn get_access_type(
        &mut self,
        checklist: Uuid,
        user: User,
    ) -> Result<AccessType, ArgentError> {
        let row = sqlx::query(
            "SELECT access_type
                FROM checklist_access
                WHERE checklist = $1
                AND argent_user = $2",
        )
        .bind(checklist)
        .bind(user.id)
        .fetch_optional(&mut *self.db)
        .await?;

        let access_type = match row {
            Some(row) => row.try_get("access_type")?,
            None => AccessType::None,
        };
        Ok(access_type)
    }

    pub async fn remove_useraccess(
        &mut self,
        checklist: Uuid,
        user_id: Uuid,
    ) -> Result<(), ArgentError> {
        sqlx::query(
            "DELETE FROM checklist_access
            WHERE checklist = $1
            AND argent_user = $2",
        )
        .bind(checklist)
        .bind(user_id)
        .execute(&mut *self.db)
        .await?;
        Ok(())
    }

    pub async fn get_users_access_for_checklist(
        &mut self,
        checklist: Uuid,
    ) -> Result<Vec<UserAccess>, ArgentError> {
        let user_accesses = sqlx::query_as(
            "SELECT
                id,
                name,
                access_type
            FROM argent_users u
            LEFT JOIN checklist_access ca
            ON ca.argent_user = u.id
            WHERE ca.checklist = $1",
        )
        .bind(checklist)
        .fetch_all(&mut *self.db)
        .await?;
        Ok(user_accesses)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ChecklistStore {
    type Error = Infallible;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let db = request.guard::<Connection<ArgentDB>>().await.unwrap();
        rocket::request::Outcome::Success(ChecklistStore { db })
    }
}
