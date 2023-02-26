use rocket::{get, post, routes, Route};

use crate::{
    api::{
        auth::user_guard::AuthenticatedUser,
        helpers::{ArgentApiResult, NewData, OkData},
    },
    data::marble_game::{models::GameStatus, store::MarbleGameStore},
    error::SimpleMessage,
};

#[get("/marble-game/status")]
async fn get_status(
    user: AuthenticatedUser,
    mut marble_game_store: MarbleGameStore,
) -> ArgentApiResult<GameStatus> {
    let status = marble_game_store.get_game_status(user.get()).await?;
    ArgentApiResult::new(status)
}

#[post("/marble-game/update-highest-cleared")]
async fn update_highest_cleared(
    user: AuthenticatedUser,
    mut marble_game_store: MarbleGameStore,
) -> ArgentApiResult<SimpleMessage> {
    marble_game_store.update_highest_cleared(user.get()).await?;
    ArgentApiResult::new_ok()
}

pub fn routes() -> Vec<Route> {
    routes![get_status, update_highest_cleared]
}
