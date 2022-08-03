use std::convert::Infallible;

use rocket::request::FromRequest;
use rocket_db_pools::Connection;
use sqlx::{query, query_as};

use crate::data::users::models::User;
use crate::data::ArgentDB;
use crate::{api::helpers::ArgentResult, error::ArgentError};

use super::models::GameStatus;

pub struct MarbleGameStore {
    db: Connection<ArgentDB>,
}

impl MarbleGameStore {
    pub async fn get_game_status(&mut self, user: User) -> Result<Option<GameStatus>, ArgentError> {
        let game_status = query_as(
            "SELECT
                    argent_user,
                    highest_cleared
                FROM marble_game_status
                WHERE argent_user = $1",
        )
        .bind(user.id)
        .fetch_optional(&mut *self.db)
        .await?;
        Ok(game_status)
    }

    pub async fn update_highest_cleared(&mut self, user: User) -> ArgentResult<()> {
        query(
            "INSERT INTO marble_game_status (
                argent_user,
                highest_cleared
            )
            VALUES ($1, 1)
            ON CONFLICT (argent_user) DO UPDATE
                SET highest_cleared = marble_game_status.highest_cleared + 1;",
        )
        .bind(user.id)
        .execute(&mut *self.db)
        .await
        .map(|_| ())?;
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MarbleGameStore {
    type Error = Infallible;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let db = request.guard::<Connection<ArgentDB>>().await.unwrap();
        rocket::request::Outcome::Success(MarbleGameStore { db })
    }
}
