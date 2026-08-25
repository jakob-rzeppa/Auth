use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::authorize::error::{AuthorizeError, fatal::FatalAuthorizeError};

/// The OAuth 2.0 Authorization Request, per RFC 6749 Section 4.1.1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizeRequest {
    pub response_type: String,
    pub client_id: Uuid,
    pub redirect_uri: String,
    pub scope: Vec<String>,
    pub state: String,

    pub user_email: String,
    pub user_password: String,
}

#[derive(Debug, Deserialize)]
struct RawAuthorizeRequest {
    response_type: Option<String>,
    client_id: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,

    user_email: Option<String>,
    user_password: Option<String>,
}

impl<S> FromRequest<S> for AuthorizeRequest
where
    S: Send + Sync,
{
    type Rejection = AuthorizeError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state).await.map_err(|_| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The request body is missing or invalid.",
            ))
        })?;

        let raw: RawAuthorizeRequest = serde_urlencoded::from_bytes(&bytes).map_err(|_| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The request body is missing or invalid.",
            ))
        })?;

        let response_type = raw.response_type.ok_or_else(|| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The response_type is missing.",
            ))
        })?;

        let client_id = raw
            .client_id
            .ok_or_else(|| {
                AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                    "The client_id is missing.",
                ))
            })?
            .parse::<Uuid>()
            .map_err(|_| {
                AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                    "The client_id is missing.",
                ))
            })?;

        let redirect_uri = raw.redirect_uri.ok_or_else(|| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The redirect_uri is missing.",
            ))
        })?;

        let scope = raw
            .scope
            .map(|scope| scope.split_whitespace().map(String::from).collect())
            .unwrap_or_default();

        let state = raw.state.ok_or_else(|| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The state is missing.",
            ))
        })?;

        let user_email = raw.user_email.ok_or_else(|| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The user_email is missing.",
            ))
        })?;

        let user_password = raw.user_password.ok_or_else(|| {
            AuthorizeError::Fatal(FatalAuthorizeError::new_invalid_request(
                "The user_password is missing.",
            ))
        })?;

        Ok(AuthorizeRequest {
            response_type,
            client_id,
            redirect_uri,
            scope,
            state,
            user_email,
            user_password,
        })
    }
}
