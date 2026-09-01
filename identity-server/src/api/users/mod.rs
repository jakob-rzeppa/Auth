mod create;
mod delete;
mod get;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::post(create::create_user_endpoint))
        .route(
            "/{user_id}",
            axum::routing::delete(delete::delete_user_endpoint),
        )
        .route("/{user_id}", axum::routing::get(get::get_user_endpoint))
}
