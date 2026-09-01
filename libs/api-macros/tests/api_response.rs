use api_macros::ApiResponse;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[ApiResponse(axum::http::StatusCode::CREATED)]
pub struct CreateUserResponse {
    pub id: String,
}

#[ApiResponse(StatusCode::OK)]
pub struct ListResponse<T: serde::Serialize> {
    pub data: Vec<T>,
}

async fn body_of(response: impl IntoResponse) -> (StatusCode, String) {
    let response = response.into_response();
    let status = response.status();

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn serialises_the_struct_with_the_given_status() {
    let (status, body) = body_of(CreateUserResponse {
        id: "abc".to_string(),
    })
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body, r#"{"id":"abc"}"#);
}

#[tokio::test]
async fn works_with_generics() {
    let (status, body) = body_of(ListResponse {
        data: vec!["a", "b"],
    })
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"data":["a","b"]}"#);
}
