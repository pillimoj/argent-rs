use rocket::{
    log::private::{error, info},
    Build, Rocket,
};
use rocket_db_pools::Database;

use crate::data::{
    users::{
        models::{User, UserRole},
        store::UsersStore,
    },
    ArgentDB,
};

pub fn load_debug_env() {
    if cfg!(debug_assertions) {
        info!("loading debug environment...");
        std::fs::read_to_string("./debug.env")
            .unwrap()
            .split('\n')
            .for_each(|it| {
                let kv = if it.trim().is_empty() || it.starts_with('#') {
                    None
                } else {
                    it.split_once('=')
                };
                if let Some((key, value)) = kv {
                    std::env::set_var(key, value);
                }
            })
    }
}

pub async fn init_dev_admin(rocket: Rocket<Build>) -> Rocket<Build> {
    if cfg!(debug_assertions) {
        match ArgentDB::fetch(&rocket) {
            Some(database) => match database.acquire().await {
                Ok(mut conn) => {
                    match UsersStore::has_user_for_email(&mut conn, "eric.o.berglund@gmail.com")
                        .await
                    {
                        Ok(false) => {
                            info!("Adding dev admin user");
                            let res = UsersStore::add_user_conn(
                                &mut conn,
                                User {
                                    id: uuid::Uuid::new_v4(),
                                    email: String::from("eric.o.berglund@gmail.com"),
                                    name: String::from("Eric"),
                                    role: UserRole::Admin,
                                },
                            )
                            .await;
                            if let Err(err) = res {
                                error!("Adding dev user: Db error when adding users: {}", err);
                            }
                        }
                        Ok(true) => {}
                        Err(err) => {
                            error!("Adding dev user: Db error when checking for user: {}", err)
                        }
                    }
                }
                Err(err) => error!("Adding dev user: No database connection: {}", err),
            },
            None => error!("Adding dev user: No database"),
        }
    };
    return rocket;
}
