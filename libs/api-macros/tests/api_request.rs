use api_macros::{ApiErrorResponse, ApiRequest};
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[ApiErrorResponse]
pub enum UpdateUserErrorResponse {
    #[status_code(StatusCode::BAD_REQUEST)]
    #[error("invalid_request")]
    #[description("Invalid request body.")]
    InvalidBody,
}

#[ApiRequest(UpdateUserErrorResponse::InvalidBody)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
}

fn request_with(body: &'static str) -> axum::extract::Request {
    axum::extract::Request::builder()
        .body(axum::body::Body::from(body))
        .unwrap()
}

#[tokio::test]
async fn deserialises_a_valid_body() {
    let result = <UpdateUserRequest as FromRequest<()>>::from_request(
        request_with(r#"{"email":"a@b.com"}"#),
        &(),
    )
    .await;

    let req = match result {
        Ok(req) => req,
        Err(_) => panic!("expected a valid body to deserialise"),
    };
    assert_eq!(req.email.as_deref(), Some("a@b.com"));
}

#[tokio::test]
async fn rejects_an_invalid_body_with_the_given_error() {
    let result = <UpdateUserRequest as FromRequest<()>>::from_request(
        request_with("not json"),
        &(),
    )
    .await;

    let rejection = match result {
        Ok(_) => panic!("expected an invalid body to be rejected"),
        Err(rejection) => rejection,
    };
    assert_eq!(rejection.into_response().status(), StatusCode::BAD_REQUEST);
}
