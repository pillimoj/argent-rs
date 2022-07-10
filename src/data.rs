use rocket::{error, fairing, Build, Rocket};
use rocket_db_pools::Database;

pub mod checklists {
    pub mod models;
    pub mod store;
}

pub mod users {
    pub mod models;
    pub mod store;
}

#[derive(Database)]
#[database("argent")]
pub struct ArgentDB(sqlx::PgPool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match ArgentDB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
