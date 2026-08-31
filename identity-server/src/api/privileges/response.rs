use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreatePrivilegeResponse {
    pub id: String,
}

impl IntoResponse for CreatePrivilegeResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, axum::Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_privilege_response_into_response() {
        let test_id = "123e4567-e89b-12d3-a456-426614174000";
        let response = CreatePrivilegeResponse {
            id: test_id.to_string(),
        };

        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(axum_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(
            body_text,
            "{\"id\":\"123e4567-e89b-12d3-a456-426614174000\"}"
        );
    }
}
