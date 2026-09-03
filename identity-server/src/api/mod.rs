mod health;
mod users;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/health", axum::routing::get(health::health_endpoint))
        .nest("/users", users::router())
}
