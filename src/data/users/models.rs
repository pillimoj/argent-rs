use rocket::serde::{Deserialize, Serialize};

use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
#[sqlx(type_name = "TEXT")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Serialize)]
pub struct UserForSharing {
    id: Uuid,
    name: String,
}
impl UserForSharing {
    pub fn from_user(user: User) -> Self {
        UserForSharing {
            id: user.id,
            name: user.name,
        }
    }
}
