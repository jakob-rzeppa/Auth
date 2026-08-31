use axum::response::Response;

use crate::api::privileges::{
    error::PrivilegeApiError, request::CreatePrivilegeRequest, response::CreatePrivilegeResponse,
};

mod error;
mod request;
mod response;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::post(create_privilege))
        .route("/{id}", axum::routing::patch(patch_privilege))
        .route("/{id}", axum::routing::delete(delete_privilege))
}

#[axum::debug_handler]
async fn create_privilege(
    request: CreatePrivilegeRequest,
) -> Result<CreatePrivilegeResponse, PrivilegeApiError> {
    todo!()
}

#[axum::debug_handler]
async fn patch_privilege() -> Result<Response, PrivilegeApiError> {
    todo!()
}

#[axum::debug_handler]
async fn delete_privilege() -> Result<Response, PrivilegeApiError> {
    todo!()
}
