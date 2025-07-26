use chrono::{Duration, NaiveDate, Utc};
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
use diesel::sql_types::{BigInt, Date, Timestamptz, Uuid as SqlUuid};

#[derive(QueryableByName)]
struct CountResult {
    #[sql_type = "BigInt"]
    count: i64,
}

#[derive(QueryableByName)]
struct DayCountResult {
    #[sql_type = "Date"]
    day: NaiveDate,
    #[sql_type = "BigInt"]
    count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyLogCount {
    pub day: NaiveDate,
    pub count: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub total_repositories: i64,
    pub enabled: i64,
    pub disabled: i64,
    pub last_cloned_repos: Vec<RepositoryModel>,
    pub daily_logs: Vec<DailyLogCount>,
    pub daily_error_logs: Vec<DailyLogCount>,
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

    // Fetch last cloned repositories
    let last_cloned_repos = repository::dsl::repository
        .filter(repository::dsl::user_id.eq(user.0.id))
        .filter(repository::dsl::last_clone_at.is_not_null())
        .order(repository::dsl::last_clone_at.desc())
        .limit(10)
        .load::<RepositoryModel>(conn)
        .unwrap_or_default();

    // Chart data: daily log counts for past 7 days (all logs)
    let logs_query = "\
        SELECT date(l.created_at) as day, count(*) as count \
        FROM repository_logs l \
        JOIN repository r ON r.id = l.repository_id \
        WHERE r.user_id = $1 AND l.created_at >= $2 \
        GROUP BY day ORDER BY day";

    let raw_daily_logs: Vec<DayCountResult> = sql_query(logs_query)
        .bind::<SqlUuid, _>(user.0.id)
        .bind::<Timestamptz, _>(week_ago)
        .load(conn)
        .unwrap_or_else(|_| vec![]);

    // Chart data: daily error log counts for past 7 days (type = 'error_clone_job')
    let errors_query = "\
        SELECT date(l.created_at) as day, count(*) as count \
        FROM repository_logs l \
        JOIN repository r ON r.id = l.repository_id \
        WHERE r.user_id = $1 AND l.created_at >= $2 AND l.type = 'error_clone_job' \
        GROUP BY day ORDER BY day";

    let raw_daily_errors: Vec<DayCountResult> = sql_query(errors_query)
        .bind::<SqlUuid, _>(user.0.id)
        .bind::<Timestamptz, _>(week_ago)
        .load(conn)
        .unwrap_or_else(|_| vec![]);

    // Ensure 7 days even if zero counts for logs
    let mut daily_logs = Vec::new();
    for i in 0..7 {
        let date = (now - Duration::days(i)).date_naive();
        let count = raw_daily_logs
            .iter()
            .find(|r| r.day == date)
            .map(|r| r.count)
            .unwrap_or(0);
        daily_logs.push(DailyLogCount { day: date, count });
    }
    daily_logs.reverse();

    // Ensure 7 days even if zero counts for errors
    let mut daily_error_logs = Vec::new();
    for i in 0..7 {
        let date = (now - Duration::days(i)).date_naive();
        let count = raw_daily_errors
            .iter()
            .find(|r| r.day == date)
            .map(|r| r.count)
            .unwrap_or(0);
        daily_error_logs.push(DailyLogCount { day: date, count });
    }
    daily_error_logs.reverse();

    let data = DashboardData {
        total_repositories,
        enabled,
        disabled,
        last_cloned_repos,
        daily_logs,
        daily_error_logs,
    };

    Custom(
        Status::Ok,
        Json(ApiResponse::success(
            "Dashboard data fetched",
            GetDashboardDataResponse { dashboard: data },
        )),
    )
}
