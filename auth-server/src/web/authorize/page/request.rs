use axum::extract::FromRequestParts;
use serde::Deserialize;

use crate::web::authorize::page::error_response::AuthorizePageErrorResponse;

#[derive(Deserialize)]
pub struct AuthorizePageQuery {
    pub client_id: Option<String>,
    pub request_uri: Option<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for AuthorizePageQuery {
    type Rejection = AuthorizePageErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        serde_urlencoded::from_str(query).map_err(|_| AuthorizePageErrorResponse::MalformedRequest)
    }
}
