use api_macros::ApiErrorResponse;
use axum::http::StatusCode;

#[ApiErrorResponse]
pub enum CreateUserErrorResponse {
    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_request")]
    #[description("Invalid request body.")]
    InvalidBody,

    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_email")]
    #[description("Email cannot be empty.")]
    InvalidEmail,

    #[status_code(StatusCode::CONFLICT)]
    #[error("email_already_exists")]
    #[description("Email already exists.")]
    EmailAlreadyExists,

    #[status_code(StatusCode::INTERNAL_SERVER_ERROR)]
    #[error("internal_server_error")]
    #[description("An internal server error occurred.")]
    InternalServerError,
}
