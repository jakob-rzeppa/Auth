//! OpenAPI document assembly, served alongside the API by [`super::router`].

use utoipa::OpenApi;

use crate::api::health;
use crate::api::users::{create, delete, get, update};

/// Documents the `{"error": ..., "error_description": ...}` body produced by every
/// `#[ApiErrorResponse]` enum. The enums themselves can't derive `ToSchema` - their
/// variants are turned into that shape at runtime by the macro, not by serialising
/// the enum directly - so this stands in for them in the generated spec.
#[derive(utoipa::ToSchema)]
#[allow(dead_code)]
pub struct ErrorBody {
    /// Machine-readable error code, e.g. `"user_not_found"`.
    pub error: String,
    /// Human-readable description of the error.
    pub error_description: String,
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Identity Server", version = env!("CARGO_PKG_VERSION")),
    paths(
        health::health_endpoint,
        create::create_user_endpoint,
        get::get_user_endpoint,
        update::update_user_endpoint,
        delete::delete_user_endpoint,
    ),
    components(schemas(ErrorBody))
)]
pub struct ApiDoc;
