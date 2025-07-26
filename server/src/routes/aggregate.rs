use chrono::{Duration, Utc};
use diesel::prelude::*;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::db::DbConnection;
use crate::middlewares::auth::AuthGuard;
use crate::models::RepositoryModel;
use crate::schema::repository;
use crate::utils::response::ApiResponse;
use diesel::deserialize::QueryableByName;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Timestamptz};

#[derive(QueryableByName)]
struct CountResult {
    #[sql_type = "BigInt"]
    count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub total_repositories: i64,
    pub enabled: i64,
    pub disabled: i64,
    pub clone_frequency_past_day: i64,
    pub clone_frequency_past_week: i64,
    pub logs_frequency_past_day: i64,
    pub logs_frequency_past_week: i64,
    pub failed_clone_job_frequency_past_day: i64,
    pub failed_clone_job_frequency_past_week: i64,
    pub last_cloned_repos: Vec<RepositoryModel>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboardDataResponse {
    pub dashboard: DashboardData,
}

#[get("/aggregate/dashboard")]
pub fn get_dashboard_data(
    db: &State<DbConnection>,
    user: AuthGuard,
) -> Custom<Json<ApiResponse<GetDashboardDataResponse>>> {
    let conn = &mut db.get().unwrap();
    let now = Utc::now();
    let day_ago = now - Duration::days(1);
    let week_ago = now - Duration::days(7);

    // Counts for repositories
    let total_repositories = repository::dsl::repository
        .filter(repository::dsl::user_id.eq(user.0.id))
        .count()
        .get_result::<i64>(conn)
        .unwrap_or(0);

    let enabled = repository::dsl::repository
        .filter(repository::dsl::user_id.eq(user.0.id))
        .filter(repository::dsl::is_enabled.eq(true))
        .count()
        .get_result::<i64>(conn)
        .unwrap_or(0);

    let disabled = total_repositories - enabled;

    // Helper function to count logs by filter and time range
    fn count_logs(
        conn: &mut PgConnection,
        user_id: uuid::Uuid,
        log_type: Option<&str>,
        since: chrono::DateTime<Utc>,
    ) -> i64 {
        let filter = if let Some(t) = log_type {
            format!("AND l.type_ = '{}'", t)
        } else {
            "".to_string()
        };

        let query = format!(
            "SELECT count(*) as count FROM repository_logs l \
             JOIN repository r ON r.id = l.repository_id \
             WHERE r.user_id = $1 AND l.created_at >= $2 {}",
            filter
        );

        sql_query(query)
            .bind::<diesel::sql_types::Uuid, _>(user_id)
            .bind::<Timestamptz, _>(since)
            .load::<CountResult>(conn)
            .ok()
            .and_then(|mut rows| rows.pop())
            .map(|row| row.count)
            .unwrap_or(0)
    }

    let clone_frequency_past_day = count_logs(conn, user.0.id, Some("finished_clone_job"), day_ago);
    let clone_frequency_past_week =
        count_logs(conn, user.0.id, Some("finished_clone_job"), week_ago);

    let logs_frequency_past_day = count_logs(conn, user.0.id, None, day_ago);
    let logs_frequency_past_week = count_logs(conn, user.0.id, None, week_ago);

    let failed_clone_job_frequency_past_day =
        count_logs(conn, user.0.id, Some("failed_clone_job"), day_ago);
    let failed_clone_job_frequency_past_week =
        count_logs(conn, user.0.id, Some("failed_clone_job"), week_ago);

    let last_cloned_repos = repository::dsl::repository
        .filter(repository::dsl::user_id.eq(user.0.id))
        .filter(repository::dsl::last_clone_at.is_not_null())
        .order(repository::dsl::last_clone_at.desc())
        .limit(10)
        .load::<RepositoryModel>(conn)
        .unwrap_or_default();

    let data = DashboardData {
        total_repositories,
        enabled,
        disabled,
        clone_frequency_past_day,
        clone_frequency_past_week,
        logs_frequency_past_day,
        logs_frequency_past_week,
        failed_clone_job_frequency_past_day,
        failed_clone_job_frequency_past_week,
        last_cloned_repos,
    };

    Custom(
        Status::Ok,
        Json(ApiResponse::success(
            "Dashboard data fetched",
            GetDashboardDataResponse { dashboard: data },
        )),
    )
}
