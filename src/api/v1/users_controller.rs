use rocket::{get, routes, Route};

use crate::{
    api::{
        auth::user_guard::AuthenticatedUser,
        helpers::{ArgentApiResult, NewData},
    },
    data::users::{
        models::{User, UserForSharing},
        store::UsersStore,
    },
};

#[get("/me")]
fn me(user: AuthenticatedUser) -> ArgentApiResult<User> {
    ArgentApiResult::new(user.get())
}

#[get("/users")]
async fn get_all_for_sharing(
    _user: AuthenticatedUser,
    mut users_store: UsersStore,
) -> ArgentApiResult<Vec<UserForSharing>> {
    let users = users_store
        .get_all_users()
        .await?
        .into_iter()
        .map(UserForSharing::from_user)
        .collect::<Vec<_>>();
    ArgentApiResult::new(users)
}

pub fn routes() -> Vec<Route> {
    routes![me, get_all_for_sharing]
}
