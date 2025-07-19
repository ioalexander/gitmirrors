use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{State, http::CookieJar};
use serde::Deserialize;

use crate::db::DbConnection;
use crate::models::UserModel;
use crate::schema::user;

#[derive(Deserialize)]
struct UserLoginForm<'r> {
    username: &'r str,
    password: &'r str,
}

#[post("/user/login", data = "<login>")]
pub fn login(db: &State<DbConnection>, _cookie_jar: &CookieJar, login: Json<UserLoginForm<'_>>) {
    let connection = &mut db.get().expect("Failed to get DB Connection");

    let results = user::table
        .filter(user::username.eq(login.username))
        .select(UserModel::as_select())
        .load::<UserModel>(connection)
        .expect("Error loading users");

    dbg!(&results);
}
