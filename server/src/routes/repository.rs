use diesel::prelude::*;
use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::DbConnection;
use crate::middlewares::auth::AuthGuard;
use crate::models::{InsertableRepositoryModel, RepositoryLogModel, RepositoryModel};
use crate::schema::repository;
use crate::utils::response::ApiResponse;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepositoryResponse {
    pub repository: RepositoryModel,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepositoryLogsResponse {
    pub repository_logs: Vec<RepositoryLogModel>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRepositoryResponse {
    pub repository: RepositoryModel,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepositoriesResponse {
    pub repositories: Vec<RepositoryModel>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddRepositoryForm {
    #[validate(length(
        min = 3,
        max = 200,
        message = "Name length should be more than 3 characters and less than 200 characters long"
    ))]
    pub name: String,

    #[validate(length(max = 512, message = "Url should be less than 512 characters long"))]
    pub url: String,

    #[validate(length(
        min = 3,
        max = 512,
        message = "git Source should be more than 3 characters and less than 512 characters long"
    ))]
    pub git_source: String,

    #[validate(length(
        max = 512,
        message = "git Source Private Key should be less than 512 characters long"
    ))]
    pub git_source_secret_key: Option<String>,

    #[validate(length(
        min = 3,
        max = 512,
        message = "git Target should be more than 3 characters and less than 512 characters long"
    ))]
    pub git_target: String,

    #[validate(length(
        min = 3,
        message = "git Target Secret Key should be more than 3 characters long"
    ))]
    pub git_target_secret_key: String,

    #[validate(range(
        min = 60,
        max = 31_556_952,
        message = "Cloning period must be between 60 seconds and 1 year"
    ))]
    pub git_clone_period_seconds: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRepositoryResponse {
    pub created_repository: RepositoryModel,
}

#[get("/repository/<repo_id>")]
pub fn get_repository_by_id(
    db: &State<DbConnection>,
    user: AuthGuard,
    repo_id: String,
) -> Custom<Json<ApiResponse<GetRepositoryResponse>>> {
    use crate::schema::repository::dsl::*;

    let connection = &mut db.get().unwrap();

    let parsed_id = match uuid::Uuid::parse_str(&repo_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Custom(
                Status::BadRequest,
                Json(ApiResponse::error("Invalid repository ID")),
            );
        }
    };

    match repository
        .filter(id.eq(parsed_id).and(user_id.eq(user.0.id)))
        .first::<RepositoryModel>(connection)
        .optional()
    {
        Ok(Some(repo)) => Custom(
            Status::Ok,
            Json(ApiResponse::success(
                "Repository fetched successfully",
                GetRepositoryResponse { repository: repo },
            )),
        ),
        Ok(None) => Custom(
            Status::NotFound,
            Json(ApiResponse::error("Repository not found")),
        ),
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ApiResponse::error("Failed to fetch repository")),
        ),
    }
}

#[get("/repository/<repo_id>/logs")]
pub fn get_repository_logs_by_id(
    db: &State<DbConnection>,
    user: AuthGuard,
    repo_id: String,
) -> Custom<Json<ApiResponse<GetRepositoryLogsResponse>>> {
    use crate::schema::repository::dsl::*;

    let connection = &mut db.get().unwrap();

    let parsed_id = match uuid::Uuid::parse_str(&repo_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Custom(
                Status::BadRequest,
                Json(ApiResponse::error("Invalid repository ID")),
            );
        }
    };

    let repo = match repository
        .filter(id.eq(parsed_id).and(user_id.eq(user.0.id)))
        .first::<RepositoryModel>(connection)
        .optional()
    {
        Ok(Some(r)) => r,
        Ok(None) => {
            return Custom(
                Status::NotFound,
                Json(ApiResponse::error("Repository not found")),
            );
        }
        Err(_) => {
            return Custom(
                Status::InternalServerError,
                Json(ApiResponse::error("Failed to fetch repository")),
            );
        }
    };

    let logs_result = crate::schema::repository_logs::table
        .filter(crate::schema::repository_logs::repository_id.eq(repo.id))
        .order(crate::schema::repository_logs::created_at.desc())
        .load::<RepositoryLogModel>(connection);

    match logs_result {
        Ok(logs) => Custom(
            Status::Ok,
            Json(ApiResponse::success(
                "Logs fetched successfully",
                GetRepositoryLogsResponse {
                    repository_logs: logs,
                },
            )),
        ),
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ApiResponse::error("Failed to fetch logs")),
        ),
    }
}

#[delete("/repository/<repo_id>")]
pub fn delete_repository_by_id(
    db: &State<DbConnection>,
    user: AuthGuard,
    repo_id: String,
) -> Custom<Json<ApiResponse<DeleteRepositoryResponse>>> {
    use crate::schema::repository::dsl::*;

    let connection = &mut db.get().unwrap();

    let parsed_id = match uuid::Uuid::parse_str(&repo_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Custom(
                Status::BadRequest,
                Json(ApiResponse::error("Invalid repository ID")),
            );
        }
    };

    match repository
        .filter(id.eq(parsed_id).and(user_id.eq(user.0.id)))
        .first::<RepositoryModel>(connection)
        .optional()
    {
        Ok(Some(repo)) => {
            if diesel::delete(repository.filter(id.eq(parsed_id)))
                .execute(connection)
                .is_err()
            {
                return Custom(
                    Status::InternalServerError,
                    Json(ApiResponse::error("Failed to delete repository")),
                );
            }

            Custom(
                Status::Ok,
                Json(ApiResponse::success(
                    "Repository deleted successfully",
                    DeleteRepositoryResponse { repository: repo },
                )),
            )
        }
        Ok(None) => Custom(
            Status::NotFound,
            Json(ApiResponse::error("Repository not found")),
        ),
        Err(_) => Custom(
            Status::InternalServerError,
            Json(ApiResponse::error("Failed to fetch repository")),
        ),
    }
}

#[get("/repository")]
pub fn get_all_repositories(
    db: &State<DbConnection>,
    user: AuthGuard,
) -> Custom<Json<ApiResponse<GetRepositoriesResponse>>> {
    let connection = &mut db.get().unwrap();

    let results = repository::table
        .filter(repository::user_id.eq(user.0.id))
        .load::<RepositoryModel>(connection)
        .unwrap();

    Custom(
        Status::Ok,
        Json(ApiResponse::success(
            "Repositories fetched successfully",
            GetRepositoriesResponse {
                repositories: results,
            },
        )),
    )
}

#[post("/repository", format = "application/json", data = "<form>")]
pub fn add_repository(
    db: &State<DbConnection>,
    user: AuthGuard,
    form: Json<AddRepositoryForm>,
) -> Custom<Json<ApiResponse<AddRepositoryResponse>>> {
    if let Err(_e) = form.validate() {
        return Custom(Status::BadRequest, Json(ApiResponse::error("Bad request")));
    }

    let connection = &mut db.get().unwrap();

    let new_repo = InsertableRepositoryModel {
        user_id: user.0.id,
        name: form.name.as_str(),
        url: Some(form.url.as_str()).filter(|s| !s.is_empty()),
        is_enabled: true,
        git_source: form.git_source.as_str(),
        git_source_secret_key: form.git_source_secret_key.as_deref(),
        git_target: form.git_target.as_str(),
        git_target_secret_key: Some(form.git_target_secret_key.as_str()),
        git_clone_period_seconds: form.git_clone_period_seconds as i32,
    };

    match diesel::insert_into(crate::schema::repository::table)
        .values(&new_repo)
        .get_result::<RepositoryModel>(connection)
    {
        Ok(inserted) => Custom(
            Status::Ok,
            Json(ApiResponse::success(
                "Repository added successfully",
                AddRepositoryResponse {
                    created_repository: inserted,
                },
            )),
        ),
        Err(_e) => Custom(
            Status::InternalServerError,
            Json(ApiResponse::error(
                "Failed to insert repository into database",
            )),
        ),
    }
}
