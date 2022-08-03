use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatus {
    pub argent_user: Uuid,
    pub highest_cleared: i32,
}
