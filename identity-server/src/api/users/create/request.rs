use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde::Deserialize;

use crate::api::users::create::error_response::CreateUserErrorResponse;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}

impl<S: Send + Sync> FromRequest<S> for CreateUserRequest {
    type Rejection = CreateUserErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| CreateUserErrorResponse::InvalidBody)?;

        serde_json::from_slice(&bytes).map_err(|_| CreateUserErrorResponse::InvalidBody)
    }
}
