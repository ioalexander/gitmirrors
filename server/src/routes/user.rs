use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct User {
    id: String,
    username: String,
}

#[get("/user/login")]
pub fn login() -> Json<User> {
    Json(User {
        id: "test".into(),
        username: "test".into(),
    })
}
