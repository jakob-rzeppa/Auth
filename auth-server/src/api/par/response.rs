use api_macros::ApiResponse;
use axum::http::{StatusCode, header};

#[ApiResponse(StatusCode::CREATED, headers(header::CACHE_CONTROL => "no-store"))]
pub struct AuthorizePushResponse {
    pub request_uri: String,
    pub expires_in: u64,
}
