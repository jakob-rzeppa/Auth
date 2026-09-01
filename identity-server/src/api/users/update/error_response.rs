use api_macros::ApiErrorResponse;
use axum::http::StatusCode;

#[ApiErrorResponse]
pub enum UpdateUserErrorResponse {
    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_user_id")]
    #[description("The provided user ID is invalid.")]
    InvalidUserId,

    #[status_code(StatusCode::NOT_FOUND)]
    #[error("user_not_found")]
    #[description("The user was not found.")]
    UserNotFound,

    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_email")]
    #[description("Email cannot be empty.")]
    InvalidEmail,

    #[status_code(StatusCode::CONFLICT)]
    #[error("email_already_exists")]
    #[description("Email already exists.")]
    EmailAlreadyExists,

    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_request")]
    #[description("Invalid request body.")]
    InvalidBody,

    #[status_code(StatusCode::INTERNAL_SERVER_ERROR)]
    #[error("internal_server_error")]
    #[description("An internal server error occurred.")]
    InternalServerError,
}
