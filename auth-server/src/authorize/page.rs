use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};

use crate::authorize::error::{AuthorizeError, fatal::FatalAuthorizeError};

#[derive(Template)]
#[template(path = "authorize.html")]
pub struct AuthorizePage {
    pub client_id: String,
    pub redirect_uri: String,
    pub state: String,
    pub scope: String,

    pub user_name: Option<String>,

    pub error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct AuthorizeQueryParams {
    client_id: Option<String>,
    redirect_uri: Option<String>,
    state: Option<String>,
    scope: Option<String>,
}

#[axum::debug_handler]
pub async fn authorize_page_endpoint(
    Query(query): Query<AuthorizeQueryParams>,
) -> Result<impl IntoResponse, AuthorizeError> {
    let Some(client_id) = query.client_id else {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new_invalid_request("Missing client_id parameter."),
        ));
    };

    let Some(redirect_uri) = query.redirect_uri else {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new_invalid_request("Missing redirect_uri parameter."),
        ));
    };

    let Some(state) = query.state else {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new_invalid_request("Missing state parameter."),
        ));
    };

    let Some(scope) = query.scope else {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new_invalid_request("Missing scope parameter."),
        ));
    };

    let page = AuthorizePage {
        client_id,
        redirect_uri,
        state,
        scope,

        user_name: None,

        error: None,
    };
    Ok(Html(
        page.render().expect("Rendering of authorize page failed."),
    ))
}
