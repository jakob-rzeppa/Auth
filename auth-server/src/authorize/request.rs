use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde::Deserialize;

use crate::authorize::error::AuthorizeError;

/// The OAuth 2.0 Authorization Request, per RFC 6749 Section 4.1.1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizeRequest {
    pub client_id: String,
    pub redirect_uri: Option<String>,
    pub scope: Option<Vec<String>>,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawAuthorizeRequest {
    response_type: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

impl<S> FromRequest<S> for AuthorizeRequest
where
    S: Send + Sync,
{
    type Rejection = AuthorizeError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state).await.map_err(|err| {
            AuthorizeError::InvalidRequest {
                description: "The request body is missing or invalid.".to_string(),
            }
        })?;

        let raw: RawAuthorizeRequest =
            serde_urlencoded::from_bytes(&bytes).map_err(|err| AuthorizeError::InvalidRequest {
                description: "The request body is invalid.".to_string(),
            })?;

        match raw.response_type.as_deref() {
            Some("code") => (),
            Some(_) => {
                return Err(AuthorizeError::UnsupportedResponseType);
            }
            None => {
                return Err(AuthorizeError::InvalidRequest {
                    description: "The response_type is missing.".to_string(),
                });
            }
        };

        let client_id = raw.client_id.ok_or(AuthorizeError::InvalidRequest {
            description: "The client_id is missing.".to_string(),
        })?;

        let scope = raw
            .scope
            .map(|scope| scope.split_whitespace().map(String::from).collect());

        Ok(AuthorizeRequest {
            client_id,
            redirect_uri: raw.redirect_uri,
            scope,
            state: raw.state,
        })
    }
}
