use api_macros::ApiErrorResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[ApiErrorResponse]
pub enum TestErrorResponse {
    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_request")]
    #[description("Invalid request body.")]
    InvalidBody,

    #[status_code(axum::http::StatusCode::CONFLICT)]
    #[error("{field}_already_exists")]
    #[description("A user with the email {email} already exists.")]
    AlreadyExists { field: String, email: String },

    #[status_code(axum::http::StatusCode::BAD_REQUEST)]
    #[error("invalid_{0}")]
    #[description("The field {0} is invalid.")]
    InvalidField(String),

    #[status_code(axum::http::StatusCode::INTERNAL_SERVER_ERROR)]
    #[error("internal_server_error")]
    #[description("An internal server error occurred.")]
    InternalServerError,
}

async fn body_of(error: TestErrorResponse) -> (StatusCode, String) {
    let response = error.into_response();
    let status = response.status();

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn unit_variant_uses_its_literal_code_and_description() {
    let (status, body) = body_of(TestErrorResponse::InvalidBody).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        body,
        r#"{"error":"invalid_request","error_description":"Invalid request body."}"#
    );
}

#[tokio::test]
async fn named_fields_interpolate_into_both_code_and_description() {
    let error = TestErrorResponse::AlreadyExists {
        field: "email".to_string(),
        email: "user@example.com".to_string(),
    };

    let (status, body) = body_of(error).await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(
        body,
        r#"{"error":"email_already_exists","error_description":"A user with the email user@example.com already exists."}"#
    );
}

#[tokio::test]
async fn tuple_fields_interpolate_by_position() {
    let error = TestErrorResponse::InvalidField("email".to_string());

    let (status, body) = body_of(error).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        body,
        r#"{"error":"invalid_email","error_description":"The field email is invalid."}"#
    );
}

#[tokio::test]
async fn status_code_expression_is_honoured() {
    let (status, _) = body_of(TestErrorResponse::InternalServerError).await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}
