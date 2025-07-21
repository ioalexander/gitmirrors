use crate::db::DbConnection;
use crate::models::UserModel;
use crate::schema::user::dsl::{session_token, user};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct AuthGuard(pub UserModel);

#[derive(Debug)]
pub enum AuthGuardError {
    MissingSessionToken,
    Unauthorized,
    GenericError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthGuardError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_token_from_cookie = match req.cookies().get("gitmirrors_session_token") {
            Some(cookie) => cookie.value(),
            None => {
                return Outcome::Error((Status::Unauthorized, AuthGuardError::MissingSessionToken));
            }
        };

        let pool = match req.rocket().state::<DbConnection>() {
            Some(pool) => pool,
            None => {
                return Outcome::Error((Status::InternalServerError, AuthGuardError::GenericError));
            }
        };

        let mut connection = match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                return Outcome::Error((Status::ServiceUnavailable, AuthGuardError::GenericError));
            }
        };

        let get_user_result = user
            .filter(session_token.eq(session_token_from_cookie))
            .select(UserModel::as_select())
            .first::<UserModel>(&mut connection);

        match get_user_result {
            Ok(dbusr) => Outcome::Success(AuthGuard(dbusr)),
            Err(diesel::result::Error::NotFound) => {
                Outcome::Error((Status::Unauthorized, AuthGuardError::Unauthorized))
            }
            Err(_) => Outcome::Error((Status::InternalServerError, AuthGuardError::GenericError)),
        }
    }
}
