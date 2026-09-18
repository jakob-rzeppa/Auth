use api_macros::ApiErrorResponse;

#[ApiErrorResponse]
pub enum AuthorizePushErrorResponse {
    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_request")]
    #[description("Invalid request body.")]
    InvalidRequestBody,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_client_id")]
    #[description("The client_id parameter is missing or invalid.")]
    InvalidClientId,

    #[status_code(axum::http::StatusCode::NOT_FOUND)]
    #[error("client_not_found")]
    #[description("The client was not found.")]
    ClientNotFound,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_redirect_uri")]
    #[description("The redirect_uri parameter is missing or invalid.")]
    InvalidRedirectUri,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_response_type")]
    #[description("The response_type parameter is missing or invalid.")]
    InvalidResponseType,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_scope")]
    #[description("The scope parameter is missing or invalid.")]
    InvalidScope,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_state")]
    #[description("The state parameter is missing or invalid.")]
    InvalidState,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_code_challenge_method")]
    #[description("The code_challenge_method parameter is missing or invalid.")]
    InvalidCodeChallengeMethod,

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_code_challenge")]
    #[description("The code_challenge parameter is missing or invalid.")]
    InvalidCodeChallenge,

    #[status_code(axum::http::StatusCode::INTERNAL_SERVER_ERROR)]
    #[error("database_error")]
    #[description("A database error occurred.")]
    DatabaseError,
}
