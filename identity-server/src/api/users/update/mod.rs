use axum::{extract::Path, response::Response};

use crate::{
    api::{
        openapi::ErrorBody,
        users::update::{error_response::UpdateUserErrorResponse, request::UpdateUserRequest},
    },
    application::users::update::{UpdateUserError, update_user},
};

pub mod error_response;
pub mod request;

#[utoipa::path(
    patch,
    path = "/v1/users/{user_id}",
    tag = "users",
    params(("user_id" = String, Path, description = "The user's ID.")),
    request_body = UpdateUserRequest,
    responses(
        (status = NO_CONTENT, description = "The user was updated."),
        (status = BAD_REQUEST, description = "The user ID or request body was invalid, or the email was empty.", body = ErrorBody,
            examples(
                ("invalid_user_id" = (summary = "Invalid user ID", value = json!({"error": "invalid_user_id", "error_description": "The provided user ID is invalid."}))),
                ("invalid_request" = (summary = "Invalid request body", value = json!({"error": "invalid_request", "error_description": "Invalid request body."}))),
                ("invalid_email" = (summary = "Empty email", value = json!({"error": "invalid_email", "error_description": "Email cannot be empty."}))),
            ),
        ),
        (status = NOT_FOUND, description = "No user with this ID exists.", body = ErrorBody,
            example = json!({"error": "user_not_found", "error_description": "The user was not found."}),
        ),
        (status = CONFLICT, description = "A user with this email already exists.", body = ErrorBody,
            example = json!({"error": "email_already_exists", "error_description": "Email already exists."}),
        ),
        (status = INTERNAL_SERVER_ERROR, description = "An internal server error occurred.", body = ErrorBody,
            example = json!({"error": "internal_server_error", "error_description": "An internal server error occurred."}),
        ),
    ),
)]
pub async fn update_user_endpoint(
    Path(user_id): Path<String>,
    UpdateUserRequest { email }: UpdateUserRequest,
) -> Result<Response, UpdateUserErrorResponse> {
    let Ok(user_id) = uuid::Uuid::parse_str(&user_id) else {
        return Err(UpdateUserErrorResponse::InvalidUserId);
    };

    update_user(user_id, email).await.map_err(|err| match err {
        UpdateUserError::UserNotFound => UpdateUserErrorResponse::UserNotFound,
        UpdateUserError::InvalidEmailFormat => UpdateUserErrorResponse::InvalidEmail,
        UpdateUserError::EmailAlreadyExists => UpdateUserErrorResponse::EmailAlreadyExists,
        UpdateUserError::DatabaseError => UpdateUserErrorResponse::InternalServerError,
    })?;

    Ok(Response::builder()
        .status(204)
        .body(axum::body::Body::empty())
        .unwrap())
}
