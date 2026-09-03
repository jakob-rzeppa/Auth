use api_macros::ApiResponse;
use axum::http::StatusCode;

#[ApiResponse(StatusCode::OK)]
pub struct HealthResponse {
    pub status: String,
}
