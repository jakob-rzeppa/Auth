mod health;
mod openapi;
mod users;

use utoipa::OpenApi;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/health", axum::routing::get(health::health_endpoint))
        .nest("/users", users::router())
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url(
            "/api-docs/openapi.json",
            openapi::ApiDoc::openapi(),
        ))
}
