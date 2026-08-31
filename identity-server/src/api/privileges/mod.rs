use axum::response::Response;

use crate::{
    api::privileges::{
        error::PrivilegeApiError, request::CreatePrivilegeRequest,
        response::CreatePrivilegeResponse,
    },
    application::privileges::create::{ApplicationPrivilegeError, create_privilege},
    domain::privilege::PrivilegeError,
    persistence::DatabaseError,
};

mod error;
mod request;
mod response;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::post(create_privilege_endpoint))
        .route("/{id}", axum::routing::patch(patch_privilege))
        .route("/{id}", axum::routing::delete(delete_privilege))
}

#[axum::debug_handler]
async fn create_privilege_endpoint(
    CreatePrivilegeRequest { name, description }: CreatePrivilegeRequest,
) -> Result<CreatePrivilegeResponse, PrivilegeApiError> {
    let id = create_privilege(&name, &description)
        .await
        .map_err(|e| match e {
            ApplicationPrivilegeError::DomainError(PrivilegeError::EmptyName) => {
                PrivilegeApiError::EmptyName
            }
            ApplicationPrivilegeError::DomainError(PrivilegeError::NameTooLong) => {
                PrivilegeApiError::NameTooLong
            }
            ApplicationPrivilegeError::DomainError(PrivilegeError::InvalidNameFormat) => {
                PrivilegeApiError::InvalidNameFormat
            }
            ApplicationPrivilegeError::DomainError(PrivilegeError::EmptyDescription) => {
                PrivilegeApiError::EmptyDescription
            }
            ApplicationPrivilegeError::DomainError(PrivilegeError::EmptyUuid) => {
                PrivilegeApiError::InternalError
            }
            ApplicationPrivilegeError::DatabaseError(DatabaseError::DuplicateName) => {
                PrivilegeApiError::DuplicateName
            }
            ApplicationPrivilegeError::DatabaseError(_) => PrivilegeApiError::InternalError,
        })?;

    Ok(CreatePrivilegeResponse { id: id.to_string() })
}

#[axum::debug_handler]
async fn patch_privilege() -> Result<Response, PrivilegeApiError> {
    todo!()
}

#[axum::debug_handler]
async fn delete_privilege() -> Result<Response, PrivilegeApiError> {
    todo!()
}
