mod par;

use axum::{Router, routing::post};

use crate::api::par::authorize_push_endpoint;

pub fn router() -> Router {
    Router::new().route("/par", post(authorize_push_endpoint))
}
