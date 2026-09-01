use api_macros::ApiResponse;
use axum::http::StatusCode;

#[ApiResponse(StatusCode::CREATED)]
pub struct CreateUserResponse {
    pub id: String,
}
