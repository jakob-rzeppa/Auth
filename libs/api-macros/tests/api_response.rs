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

#[ApiResponse(StatusCode::CREATED, headers(axum::http::header::CACHE_CONTROL => "no-store"))]
pub struct CachedResponse {
    pub id: String,
}

#[ApiResponse(
    StatusCode::OK,
    headers(
        axum::http::header::CACHE_CONTROL => "no-store",
        axum::http::header::PRAGMA => "no-cache",
    )
)]
pub struct MultiHeaderResponse<T: serde::Serialize> {
    pub data: Vec<T>,
}

#[tokio::test]
async fn sets_the_given_header() {
    let response = CachedResponse {
        id: "abc".to_string(),
    }
    .into_response();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["content-type"], "application/json");
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(&bytes[..], br#"{"id":"abc"}"#);
}

#[tokio::test]
async fn sets_multiple_headers_and_works_with_generics() {
    let response = MultiHeaderResponse { data: vec![1, 2] }.into_response();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["pragma"], "no-cache");
}

#[tokio::test]
async fn sets_no_extra_headers_when_the_argument_is_omitted() {
    let response = CreateUserResponse {
        id: "abc".to_string(),
    }
    .into_response();

    assert!(response.headers().get("cache-control").is_none());
}
