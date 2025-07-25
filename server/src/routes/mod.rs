pub mod repository;
pub mod user;

use rocket::Route;
pub fn routes() -> Vec<Route> {
    routes![
        user::login,
        user::me,
        user::change_password,
        repository::get_all_repositories,
        repository::add_repository,
        repository::get_repository_by_id,
        repository::delete_repository_by_id,
        repository::get_repository_logs_by_id
    ]
}
