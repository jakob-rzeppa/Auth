mod authorize;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(authorize::router())
}
