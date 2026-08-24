use axum::response::IntoResponse;

use crate::error::HttpError;

pub enum AuthorizeError {
    InvalidRequest { description: String },
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
}

impl Into<HttpError> for AuthorizeError {
    fn into(self) -> HttpError {
        match self {
            AuthorizeError::InvalidRequest { description } => {
                HttpError::new(400, "invalid_request".to_string(), description)
            }
            AuthorizeError::UnauthorizedClient => HttpError::new(
                401,
                "unauthorized_client".to_string(),
                "The client is not authorized to request an authorization code using this method."
                    .to_string(),
            ),
            AuthorizeError::AccessDenied => HttpError::new(
                403,
                "access_denied".to_string(),
                "The resource owner or authorization server denied the request.".to_string(),
            ),
            AuthorizeError::UnsupportedResponseType => {
                HttpError::new(400, "unsupported_response_type".to_string(), "The authorization server does not support obtaining an authorization code using this method.".to_string())
            }
            AuthorizeError::InvalidScope => {
                HttpError::new(400, "invalid_scope".to_string(), "The requested scope is invalid, unknown, or malformed.".to_string())
            }
            AuthorizeError::ServerError => {
                HttpError::new(500, "server_error".to_string(), "Internal server error.".to_string())
            }
        }
    }
}

impl IntoResponse for AuthorizeError {
    fn into_response(self) -> axum::response::Response {
        let http_error: HttpError = self.into();
        http_error.into_response()
    }
}
