pub mod repository;
pub mod user;

use rocket::Route;
pub fn routes() -> Vec<Route> {
    routes![
        user::login,
        user::me,
        user::change_password,
        repository::get_all_repositories,
        repository::add_repository
    ]
}
