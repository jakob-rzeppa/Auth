use askama::Template;
use axum::{
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use url::Url;

use crate::application::authorization_code::error::{
    AuthCodeError, FatalAuthCodeError, RedirectableAuthCodeError,
};

pub enum AuthorizeSubmitRedirectErrorResponse {
    InvalidCodeChallenge,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
}

pub enum AuthorizeSubmitPageErrorResponse {
    MalformedRequest,
    ClientNotFound,
    ClientIdMismatch,
    InvalidRedirectUri,
    InvalidState,
    RequestNotFound,
    ServerError,
}

pub enum AuthorizeSubmitErrorResponse {
    Redirect {
        error: AuthorizeSubmitRedirectErrorResponse,
        redirect_uri: String,
        state: String,
    },
    Page {
        error: AuthorizeSubmitPageErrorResponse,
    },
}

impl From<AuthCodeError> for AuthorizeSubmitErrorResponse {
    fn from(error: AuthCodeError) -> Self {
        match error {
            AuthCodeError::Fatal { error } => AuthorizeSubmitErrorResponse::Page {
                error: match error {
                    FatalAuthCodeError::ClientNotFound => {
                        AuthorizeSubmitPageErrorResponse::ClientNotFound
                    }
                    FatalAuthCodeError::ClientIdMismatch => {
                        AuthorizeSubmitPageErrorResponse::ClientIdMismatch
                    }
                    FatalAuthCodeError::InvalidRedirectUri => {
                        AuthorizeSubmitPageErrorResponse::InvalidRedirectUri
                    }
                    FatalAuthCodeError::InvalidState => {
                        AuthorizeSubmitPageErrorResponse::InvalidState
                    }
                    FatalAuthCodeError::PushedRequestNotFound => {
                        AuthorizeSubmitPageErrorResponse::RequestNotFound
                    }
                    FatalAuthCodeError::DatabaseError => {
                        AuthorizeSubmitPageErrorResponse::ServerError
                    }
                },
            },
            AuthCodeError::Redirectable {
                error,
                redirect_uri,
                state,
            } => AuthorizeSubmitErrorResponse::Redirect {
                error: match error {
                    RedirectableAuthCodeError::InvalidResponseType => {
                        AuthorizeSubmitRedirectErrorResponse::UnsupportedResponseType
                    }
                    RedirectableAuthCodeError::InvalidScope => {
                        AuthorizeSubmitRedirectErrorResponse::InvalidScope
                    }
                    RedirectableAuthCodeError::InvalidCodeChallengeMethod
                    | RedirectableAuthCodeError::InvalidCodeChallenge => {
                        AuthorizeSubmitRedirectErrorResponse::InvalidCodeChallenge
                    }
                    RedirectableAuthCodeError::DatabaseError => {
                        AuthorizeSubmitRedirectErrorResponse::ServerError
                    }
                },
                redirect_uri,
                state,
            },
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct AuthorizeSubmitErrorPage {
    error: String,
    error_description: String,
}

impl IntoResponse for AuthorizeSubmitPageErrorResponse {
    fn into_response(self) -> Response {
        let (status_code, error, error_description) = match self {
            AuthorizeSubmitPageErrorResponse::MalformedRequest => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The request is malformed.",
            ),
            AuthorizeSubmitPageErrorResponse::ClientNotFound => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The client_id parameter does not match any registered client.",
            ),
            AuthorizeSubmitPageErrorResponse::ClientIdMismatch => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The client_id parameter does not match the client_id of the authorization request.",
            ),
            AuthorizeSubmitPageErrorResponse::InvalidRedirectUri => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The redirect_uri of the authorization request is not registered for the client.",
            ),
            AuthorizeSubmitPageErrorResponse::InvalidState => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The state of the authorization request is invalid.",
            ),
            AuthorizeSubmitPageErrorResponse::RequestNotFound => (
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "The request_uri parameter does not match a pending authorization request, or it has expired.",
            ),
            AuthorizeSubmitPageErrorResponse::ServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "An unexpected error occurred while processing the request.",
            ),
        };

        let page = AuthorizeSubmitErrorPage {
            error: error.to_string(),
            error_description: error_description.to_string(),
        };

        match page.render() {
            Ok(html) => (status_code, Html(html)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl AuthorizeSubmitRedirectErrorResponse {
    fn code(&self) -> &'static str {
        match self {
            AuthorizeSubmitRedirectErrorResponse::InvalidCodeChallenge => "invalid_request",
            AuthorizeSubmitRedirectErrorResponse::UnsupportedResponseType => "invalid_request",
            AuthorizeSubmitRedirectErrorResponse::InvalidScope => "invalid_scope",
            AuthorizeSubmitRedirectErrorResponse::ServerError => "server_error",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            AuthorizeSubmitRedirectErrorResponse::InvalidCodeChallenge => {
                "The code challenge is invalid."
            }
            AuthorizeSubmitRedirectErrorResponse::UnsupportedResponseType => {
                "The response type is unsupported."
            }
            AuthorizeSubmitRedirectErrorResponse::InvalidScope => "The scope is invalid.",
            AuthorizeSubmitRedirectErrorResponse::ServerError => "A unexpected error occured.",
        }
    }
}

impl IntoResponse for AuthorizeSubmitErrorResponse {
    fn into_response(self) -> Response {
        match self {
            AuthorizeSubmitErrorResponse::Page { error } => error.into_response(),
            AuthorizeSubmitErrorResponse::Redirect {
                error,
                redirect_uri,
                state,
            } => {
                let Ok(mut url) = Url::parse(&redirect_uri) else {
                    return AuthorizeSubmitPageErrorResponse::ServerError.into_response();
                };
                url.query_pairs_mut()
                    .append_pair("error", error.code())
                    .append_pair("error_description", error.description())
                    .append_pair("state", &state);
                (StatusCode::FOUND, [(header::LOCATION, url.as_str())]).into_response()
            }
        }
    }
}
