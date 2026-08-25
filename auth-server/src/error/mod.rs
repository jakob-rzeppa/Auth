use askama::Template;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "error.html")]
struct FatalErrorPage<'a> {
    error: &'a str,
    error_description: &'a str,

    redirect_uri: Option<&'a str>,
    try_again_uri: Option<&'a str>,
}

pub struct FatalError {
    status_code: u16,
    error: String,
    error_description: String,

    redirect_uri: Option<String>,
    try_again_uri: Option<String>,
}

impl FatalError {
    pub fn new(
        status_code: u16,
        error: impl Into<String>,
        error_description: impl Into<String>,
        redirect_uri: Option<impl Into<String>>,
        try_again_uri: Option<impl Into<String>>,
    ) -> Self {
        Self {
            status_code,
            error: error.into(),
            error_description: error_description.into(),
            redirect_uri: redirect_uri.map(|uri| uri.into()),
            try_again_uri: try_again_uri.map(|uri| uri.into()),
        }
    }
}

impl IntoResponse for FatalError {
    fn into_response(self) -> Response {
        let page = FatalErrorPage {
            error: &self.error,
            error_description: &self.error_description,
            redirect_uri: self.redirect_uri.as_deref(),
            try_again_uri: self.try_again_uri.as_deref(),
        };
        (
            axum::http::StatusCode::from_u16(self.status_code).expect("Invalid HTTP status code."),
            Html(page.render().expect("Rendering of error page failed.")),
        )
            .into_response()
    }
}
