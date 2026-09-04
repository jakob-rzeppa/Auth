use crate::{
    api::{
        openapi::ErrorBody,
        users::create::{
            error_response::CreateUserErrorResponse, request::CreateUserRequest,
            response::CreateUserResponse,
        },
    },
    application::users::create::{CreateUserApplicationError, create_user},
};

pub mod error_response;
pub mod request;
pub mod response;

#[utoipa::path(
    post,
    path = "/users",
    tag = "users",
    request_body = CreateUserRequest,
    responses(
        (status = CREATED, description = "The user was created.", body = CreateUserResponse),
        (status = BAD_REQUEST, description = "The request body was invalid, or the email was empty.", body = ErrorBody,
            examples(
                ("invalid_request" = (summary = "Invalid request body", value = json!({"error": "invalid_request", "error_description": "Invalid request body."}))),
                ("invalid_email" = (summary = "Empty email", value = json!({"error": "invalid_email", "error_description": "Email cannot be empty."}))),
            ),
        ),
        (status = CONFLICT, description = "A user with this email already exists.", body = ErrorBody,
            example = json!({"error": "email_already_exists", "error_description": "Email already exists."}),
        ),
        (status = INTERNAL_SERVER_ERROR, description = "An internal server error occurred.", body = ErrorBody,
            example = json!({"error": "internal_server_error", "error_description": "An internal server error occurred."}),
        ),
    ),
)]
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
