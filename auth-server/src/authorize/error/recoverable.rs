use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::authorize::page::AuthorizePage;

pub enum RecoverableAuthorizeErrorKind {
    InvalidCredentials,
}

pub struct RecoverableAuthorizeError {
    pub kind: RecoverableAuthorizeErrorKind,

    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: String,
    pub user_name: String,
}

impl IntoResponse for RecoverableAuthorizeError {
    fn into_response(self) -> Response {
        let (status_code, error) = match self.kind {
            RecoverableAuthorizeErrorKind::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password.")
            }
        };
        let page = AuthorizePage {
            client_id: self.client_id,
            redirect_uri: self.redirect_uri,
            scope: self.scope,
            state: self.state,
            user_name: Some(self.user_name),
            error: Some(error.to_string()),
        };
        (status_code, Html(page.render().unwrap())).into_response()
    }
}
