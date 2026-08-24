use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "authorize.html")]
struct AuthorizePage;

#[axum::debug_handler]
pub async fn authorize_page_endpoint() -> impl IntoResponse {
    let page = AuthorizePage;
    page.render().unwrap()
}
