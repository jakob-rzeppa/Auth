use crate::{
    api::users::create::{
        error_response::CreateUserErrorResponse, request::CreateUserRequest,
        response::CreateUserResponse,
    },
    application::users::create::{CreateUserApplicationError, create_user},
};

pub mod error_response;
pub mod request;
pub mod response;

#[axum::debug_handler]
pub async fn create_user_endpoint(
    CreateUserRequest { email }: CreateUserRequest,
) -> Result<CreateUserResponse, CreateUserErrorResponse> {
    let id = create_user(email).await.map_err(|e| match e {
        CreateUserApplicationError::InvalidEmail => CreateUserErrorResponse::InvalidEmail,
        CreateUserApplicationError::EmailAlreadyExists => {
            CreateUserErrorResponse::EmailAlreadyExists
        }
        CreateUserApplicationError::DatabaseError => CreateUserErrorResponse::InternalServerError,
    })?;

    Ok(CreateUserResponse { id: id.to_string() })
}
