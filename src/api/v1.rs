use rocket::Route;

use crate::cors::CORS;

use self::checklists_controller::checklist_routes;
mod auth_controller;
mod checklists_controller;
mod users_controller;

pub struct ApiV1Routes {}
impl ApiV1Routes {
    pub fn get() -> Vec<Route> {
        let routes = [
            checklist_routes(),
            auth_controller::routes(),
            users_controller::routes(),
        ]
        .concat();
        CORS::add_options_method(routes)
    }
}
