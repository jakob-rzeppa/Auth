mod page;
mod push;

use axum::{
    Router,
    routing::{get, post},
};

use crate::web::authorize::{page::authorize_page_endpoint, push::authorize_push_endpoint};

pub fn router() -> Router {
    Router::new()
        .route("/par", post(authorize_push_endpoint))
        .route("/authorize", get(authorize_page_endpoint))
}
