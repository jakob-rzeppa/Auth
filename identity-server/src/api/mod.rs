mod users;

pub fn router() -> axum::Router {
    axum::Router::new().nest("/users", users::router())
}
