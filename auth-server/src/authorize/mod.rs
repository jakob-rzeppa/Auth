use axum::routing::{get, post};

use crate::{
    app_state::AppState,
    authorize::{authorize::authorize_endpoint, page::authorize_page_endpoint},
};

mod authorize;
mod error;
mod page;
mod request;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/authorize", get(authorize_page_endpoint))
        .route("/authorize", post(authorize_endpoint))
}
