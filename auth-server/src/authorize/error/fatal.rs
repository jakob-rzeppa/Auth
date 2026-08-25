use axum::response::IntoResponse;

use crate::error::FatalError;

pub enum FatalAuthorizeErrorKind {
    InvalidRequest(String),
    UnsupportedResponseType,
    UnauthorizedClient,
    InvalidScope,
    ServerError,
}

pub struct FatalAuthorizeError {
    kind: FatalAuthorizeErrorKind,

    redirect_uri: Option<String>,
    try_again_uri: Option<String>,
}

impl FatalAuthorizeError {
    pub fn new(kind: FatalAuthorizeErrorKind) -> Self {
        Self {
            kind,
            redirect_uri: None,
            try_again_uri: None,
        }
    }

    pub fn new_invalid_request(description: impl Into<String>) -> Self {
        Self::new(FatalAuthorizeErrorKind::InvalidRequest(description.into()))
    }

    pub fn with_redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub fn with_try_again_uri(mut self, try_again_uri: impl Into<String>) -> Self {
        self.try_again_uri = Some(try_again_uri.into());
        self
    }
}

impl Into<FatalError> for FatalAuthorizeError {
    fn into(self) -> FatalError {
        let (status_code, error, description) = match self.kind {
            FatalAuthorizeErrorKind::InvalidRequest(description) => {
                (400, "invalid_request", description)
            }
            FatalAuthorizeErrorKind::UnsupportedResponseType => (
                400,
                "unsupported_response_type",
                "The response type is not supported.".to_string(),
            ),
            FatalAuthorizeErrorKind::UnauthorizedClient => (
                400,
                "unauthorized_client",
                "The client is not authorized to request an authorization code.".to_string(),
            ),
            FatalAuthorizeErrorKind::InvalidScope => (
                400,
                "invalid_scope",
                "The requested scope is invalid, unknown, or malformed.".to_string(),
            ),
            FatalAuthorizeErrorKind::ServerError => (
                500,
                "server_error",
                "An internal server error occurred.".to_string(),
            ),
        };
        FatalError::new(
            status_code,
            error,
            description,
            self.redirect_uri,
            self.try_again_uri,
        )
    }
}

impl IntoResponse for FatalAuthorizeError {
    fn into_response(self) -> axum::response::Response {
        let fatal_error: FatalError = self.into();
        fatal_error.into_response()
    }
}
