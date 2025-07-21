pub mod user;

use rocket::Route;
pub fn routes() -> Vec<Route> {
    routes![user::login, user::me]
}
