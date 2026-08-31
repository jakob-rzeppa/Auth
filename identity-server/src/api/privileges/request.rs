use axum::body::Bytes;
use axum::extract::{FromRequest, Request};
use serde::Deserialize;

use crate::api::privileges::error::PrivilegeApiError;

#[derive(Deserialize)]
pub struct CreatePrivilegeRequest {
    pub name: String,
    pub description: String,
}

impl<S: Send + Sync> FromRequest<S> for CreatePrivilegeRequest {
    type Rejection = PrivilegeApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| PrivilegeApiError::InvalidBody)?;

        serde_json::from_slice(&bytes).map_err(|_| PrivilegeApiError::InvalidBody)
    }
}

pub struct PatchPrivilegeRequest {
    name: Option<String>,
    description: Option<String>,
}

impl<S: Send + Sync> FromRequest<S> for PatchPrivilegeRequest {
    type Rejection = PrivilegeApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;

    use super::*;

    mod create_privilege_request {
        use super::*;

        #[tokio::test]
        async fn parses_valid_json_body_into_create_privilege_request() {
            let req = Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"name":"read","description":"can read things"}"#,
                ))
                .unwrap();

            let result = CreatePrivilegeRequest::from_request(req, &()).await;

            match result {
                Ok(CreatePrivilegeRequest { name, description }) => {
                    assert_eq!(name, "read");
                    assert_eq!(description, "can read things");
                }
                Err(_) => panic!("expected Ok, got Err"),
            }
        }

        #[tokio::test]
        async fn rejects_malformed_json_body() {
            let req = Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from("not json"))
                .unwrap();

            let result = CreatePrivilegeRequest::from_request(req, &()).await;

            assert!(matches!(result, Err(PrivilegeApiError::InvalidBody)));
        }
    }
}
