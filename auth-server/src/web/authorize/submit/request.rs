use axum::{body::Bytes, extract::FromRequest, extract::Request};
use serde::Deserialize;
use uuid::Uuid;

use crate::web::authorize::submit::error_response::AuthorizeSubmitPageErrorResponse;

#[derive(Deserialize)]
pub struct AuthorizeSubmitRequest {
    pub request_uri: String,
    pub client_id: Uuid,
}

impl<S: Send + Sync> FromRequest<S> for AuthorizeSubmitRequest {
    type Rejection = AuthorizeSubmitPageErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|_| AuthorizeSubmitPageErrorResponse::MalformedRequest)?;

        serde_urlencoded::from_bytes(&body)
            .map_err(|_| AuthorizeSubmitPageErrorResponse::MalformedRequest)
    }
}
