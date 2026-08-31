mod privileges;

pub fn router() -> axum::Router {
    axum::Router::new().nest("/privileges", privileges::router())
}
