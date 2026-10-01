mod introspection;
mod par;
mod token;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/par", post(par::authorize_push_endpoint))
        .route("/token", post(token::token_endpoint))
        .route("/introspect", post(introspection::introspection_endpoint))
}
