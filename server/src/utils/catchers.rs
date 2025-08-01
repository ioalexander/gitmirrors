use rocket::Request;
use rocket::serde::json::Json;

use crate::utils::response::ApiResponse;

#[catch(401)]
pub fn unauthorized(_: &Request) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Unauthorized"))
}

#[catch(404)]
pub fn not_found(_: &Request) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Resource not found"))
}

#[catch(500)]
pub fn internal_error(_: &Request) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Internal server error"))
}
