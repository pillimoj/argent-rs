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
