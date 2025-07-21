use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::http::{Cookie, Status};
use rocket::serde::json::Json;
use rocket::{State, http::CookieJar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DbConnection;
use crate::middlewares::auth::AuthGuard;
use crate::models::{PublicUser, UserModel};
use crate::schema::user;
use crate::utils::crypto::generate_random_string;

#[derive(Deserialize)]
pub struct UserLoginForm<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub user: PublicUser,
}

#[derive(Serialize)]
pub struct UserChangePasswordResponse {
    pub user: PublicUser,
}

#[post("/user/login", format = "application/json", data = "<form>")]
pub fn login(
    db: &State<DbConnection>,
    cookie_jar: &CookieJar,
    form: Json<UserLoginForm<'_>>,
) -> Result<Json<UserLoginResponse>, Status> {
    let connection = &mut db.get().expect("Failed to get DB Connection");

    let get_user_result = user::table
        .filter(user::username.eq(form.username))
        .select(UserModel::as_select())
        .first::<UserModel>(connection);

    match get_user_result {
        Ok(user) => {
            if (user.username == "admin" && user.password_hash.is_none()) {
                let new_admin_session_token = set_new_session_token(user.id, connection);

                cookie_jar.add(
                    Cookie::build(("gitmirrors_session_token", new_admin_session_token))
                        .http_only(true)
                        .build(),
                );

                return Ok(Json(UserLoginResponse { user: user.into() }));
            }

            Err(Status::Forbidden)
        }
        Err(diesel::result::Error::NotFound) => Err(Status::NotFound),
        Err(_e) => Err(Status::InternalServerError),
    }
}

#[post("/user/change-password", format = "application/json", data = "<form>")]
pub fn change_password(
    db: &State<DbConnection>,
    user: AuthGuard,
    form: Json<UserLoginForm<'_>>,
) -> Result<Json<UserChangePasswordResponse>, Status> {
    let connection = &mut db.get().expect("Failed to get DB Connection");

    let get_user_result = user::table
        .filter(user::username.eq(form.username))
        .select(UserModel::as_select())
        .first::<UserModel>(connection);

    todo!()
}

pub fn set_new_session_token(
    user_id: Uuid,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> String {
    let new_token = generate_random_string(256);

    diesel::update(user::table.filter(user::id.eq(user_id)))
        .set(user::session_token.eq(&new_token))
        .execute(connection)
        .expect("Failed to set new session_token for user");

    new_token
}
