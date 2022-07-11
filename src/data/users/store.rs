use std::convert::Infallible;

use rocket::request::FromRequest;
use rocket_db_pools::Connection;
use sqlx::{pool::PoolConnection, query, query_as, Postgres};
use uuid::Uuid;

use crate::data::users::models::User;
use crate::data::ArgentDB;
use crate::{api::helpers::ArgentResult, error::ArgentError};

pub struct UsersStore {
    pub(crate) db: Connection<ArgentDB>,
}

impl UsersStore {
    pub async fn get_user_for_email(&mut self, email: &str) -> Result<User, ArgentError> {
        let user = query_as(
            "SELECT
                    id,
                    name,
                    email,
                    role
                FROM argent_users
                WHERE email = $1",
        )
        .bind(email)
        .fetch_one(&mut *self.db)
        .await?;
        Ok(user)
    }

    pub async fn has_user_for_email(
        conn: &mut PoolConnection<Postgres>,
        email: &str,
    ) -> Result<bool, ArgentError> {
        let user = query(
            "SELECT
                    id,
                    email
                FROM argent_users
                WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&mut *conn)
        .await
        .unwrap();
        Ok(user.is_some())
    }

    pub async fn get_user(&mut self, id: Uuid) -> Result<User, ArgentError> {
        let user = query_as(
            "SELECT
                    id,
                    name,
                    email,
                    role
                FROM argent_users
                WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&mut *self.db)
        .await?;
        Ok(user)
    }

    pub async fn get_all_users(&mut self) -> Result<Vec<User>, ArgentError> {
        let users = query_as(
            "SELECT
                id,
                name,
                email,
                role
            FROM argent_users",
        )
        .fetch_all(&mut *self.db)
        .await?;
        Ok(users)
    }

    pub async fn add_user_conn(
        conn: &mut PoolConnection<Postgres>,
        user: User,
    ) -> ArgentResult<()> {
        query(
            "INSERT INTO argent_users (
            id,
            name,
            email,
            role
        )
        VALUES($1, $2, $3, $4)",
        )
        .bind(user.id)
        .bind(user.name)
        .bind(user.email)
        .bind(user.role)
        .execute(&mut *conn)
        .await
        .map(|_| ())?;
        Ok(())
    }

    pub async fn add_user(&mut self, user: User) -> ArgentResult<()> {
        Self::add_user_conn(&mut *self.db, user).await
    }

    pub async fn delete_user(&mut self, user_id: Uuid) -> ArgentResult<()> {
        query(
            "
            DELETE FROM argent_users
            WHERE id = $1
        ",
        )
        .bind(user_id)
        .execute(&mut *self.db)
        .await
        .map(|_| ())?;
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UsersStore {
    type Error = Infallible;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let db = request.guard::<Connection<ArgentDB>>().await.unwrap();
        rocket::request::Outcome::Success(UsersStore { db })
    }
}
