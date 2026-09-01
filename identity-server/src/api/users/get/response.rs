use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::domain::projection::user::FullUserProjection;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub data: FullUserProjection,
}

impl IntoResponse for GetUserResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, axum::Json(self)).into_response()
    }
}
