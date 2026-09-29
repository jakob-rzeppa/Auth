mod page;
mod submit;

use axum::{Router, routing::get};

use crate::web::authorize::{page::authorize_page_endpoint, submit::authorize_submit_endpoint};

pub fn router() -> Router {
    Router::new().route(
        "/authorize",
        get(authorize_page_endpoint).post(authorize_submit_endpoint),
    )
}
