use api_macros::ApiResponse;
use axum::http::StatusCode;

#[ApiResponse(StatusCode::CREATED)]
#[derive(utoipa::ToSchema)]
pub struct CreateUserResponse {
    pub id: String,
}
