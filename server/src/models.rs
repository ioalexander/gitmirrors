use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub password_hash: Option<String>,
    pub session_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for PublicUser {
    fn from(user: UserModel) -> Self {
        PublicUser {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(
    Queryable, Selectable, Identifiable, Associations, PartialEq, Debug, Serialize, Deserialize,
)]
#[diesel(table_name = crate::schema::repository)]
#[diesel(belongs_to(UserModel, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct RepositoryModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub url: Option<String>,
    pub is_enabled: bool,
    pub git_source: String,
    pub git_source_secret_key: Option<String>,
    pub git_target: String,
    pub git_target_secret_key: Option<String>,
    pub git_clone_period_seconds: i32,
    pub last_clone_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::repository)]
pub struct InsertableRepositoryModel<'a> {
    pub user_id: Uuid,
    pub name: &'a str,
    pub url: Option<&'a str>,
    pub is_enabled: bool,
    pub git_source: &'a str,
    pub git_source_secret_key: Option<&'a str>,
    pub git_target: &'a str,
    pub git_target_secret_key: Option<&'a str>,
    pub git_clone_period_seconds: i32,
}

#[derive(Debug, Queryable, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::repository_logs)]
#[diesel(belongs_to(RepositoryModel, foreign_key = repository_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct RepositoryLogModel {
    pub id: Uuid,
    pub repository_id: Uuid,

    #[diesel(sql_type = Varchar)]
    pub type_: String,

    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::repository_logs)]
pub struct InsertableRepositoryLogModel<'a> {
    pub repository_id: Uuid,
    pub type_: &'a str,
    pub message: &'a str,
}
