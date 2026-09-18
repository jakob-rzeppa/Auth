use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "authorize.html")]
pub struct AuthorizePageResponse {
    pub client_name: String,
    pub scope: String,
    pub client_id: Uuid,
    pub request_uri: String,
}

impl IntoResponse for AuthorizePageResponse {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => (StatusCode::OK, Html(html)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
