use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::http::{Cookie, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{State, http::CookieJar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::db::DbConnection;
use crate::middlewares::auth::AuthGuard;
use crate::models::{PublicUser, UserModel};
use crate::schema::user;
use crate::utils::crypto::{generate_random_string, hash_password};
use crate::utils::response::ApiResponse;

#[derive(Deserialize, Validate)]
pub struct UserLoginForm<'r> {
    #[validate(length(min = 3, max = 32))]
    username: &'r str,
    #[validate(length(min = 8, max = 50))]
    password: &'r str,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub user: PublicUser,
}

#[derive(Deserialize, Validate)]
pub struct UserChangePasswordForm<'r> {
    #[validate(length(min = 8, max = 50))]
    password: &'r str,
}

#[derive(Serialize)]
pub struct UserChangePasswordResponse {
    pub user: PublicUser,
}

#[derive(Serialize)]
pub struct UserMeResponse {
    pub user: PublicUser,
}

#[post("/user/login", format = "application/json", data = "<form>")]
pub fn login(
    db: &State<DbConnection>,
    cookie_jar: &CookieJar,
    form: Json<UserLoginForm<'_>>,
) -> Custom<Json<ApiResponse<UserLoginResponse>>> {
    if let Err(_e) = form.validate() {
        return Custom(Status::BadRequest, Json(ApiResponse::error("Bad request")));
    }

    let connection = &mut db.get().expect("Failed to get DB Connection");

    let get_user_result = user::table
        .filter(user::username.eq(form.username))
        .select(UserModel::as_select())
        .first::<UserModel>(connection);

    match get_user_result {
        Ok(user) => {
            if user.username == "admin" && user.password_hash.is_none() {
                let new_admin_session_token = set_new_session_token(user.id, connection);

                cookie_jar.add(
                    Cookie::build(("gitmirrors_session_token", new_admin_session_token))
                        .http_only(true)
                        .build(),
                );

                return Custom(
                    Status::Ok,
                    Json(ApiResponse::success(
                        "Login successful",
                        UserLoginResponse { user: user.into() },
                    )),
                );
            }

            Custom(
                Status::Forbidden,
                Json(ApiResponse::error("Access forbidden")),
            )
        }
        Err(diesel::result::Error::NotFound) => {
            Custom(Status::NotFound, Json(ApiResponse::error("User not found")))
        }
        Err(_e) => Custom(
            Status::InternalServerError,
            Json(ApiResponse::error("Database error")),
        ),
    }
}

#[get("/user/me")]
pub fn me(db: &State<DbConnection>, user: AuthGuard) -> Custom<Json<ApiResponse<UserMeResponse>>> {
    match &user.0.session_token {
        Some(s) => s.clone(),
        None => {
            return Custom(
                Status::Unauthorized,
                Json(ApiResponse::error("Unauthorized")),
            );
        }
    };

    Custom(
        Status::Ok,
        Json(ApiResponse::success(
            "Success",
            UserMeResponse {
                user: user.0.into(),
            },
        )),
    )
}

#[post("/user/change-password", format = "application/json", data = "<form>")]
pub fn change_password(
    db: &State<DbConnection>,
    user: AuthGuard,
    form: Json<UserChangePasswordForm<'_>>,
) -> Custom<Json<ApiResponse<UserChangePasswordResponse>>> {
    if let Err(_e) = form.validate() {
        return Custom(Status::BadRequest, Json(ApiResponse::error("Bad request")));
    }

    let connection = &mut db.get().expect("Failed to get DB Connection");

    let new_password_hash = hash_password(form.password).unwrap();

    diesel::update(user::table.filter(user::id.eq(user.0.id)))
        .set(user::password_hash.eq(new_password_hash))
        .execute(connection)
        .unwrap();

    Custom(
        Status::Ok,
        Json(ApiResponse::success(
            "Success",
            UserChangePasswordResponse {
                user: user.0.into(),
            },
        )),
    )
}

pub fn set_new_session_token(
    user_id: Uuid,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> String {
    let new_token = generate_random_string(256);

    diesel::update(user::table.filter(user::id.eq(user_id)))
        .set(user::session_token.eq(&new_token))
        .execute(connection)
        .unwrap();

    new_token
}
