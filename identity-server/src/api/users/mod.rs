mod create;

pub fn router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::post(create::create_user_endpoint))
}
