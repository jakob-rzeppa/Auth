use axum::response::IntoResponse;

pub enum PrivilegeApiError {
    EmptyName,
    NameTooLong,
    InvalidNameFormat,
    EmptyDescription,
    DuplicateName,
    InvalidBody,
    InternalError,
}

impl IntoResponse for PrivilegeApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code, error_message) = match self {
            PrivilegeApiError::EmptyName => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Name cannot be empty.",
            ),
            PrivilegeApiError::NameTooLong => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Name is too long.",
            ),
            PrivilegeApiError::InvalidNameFormat => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Name format is invalid. It must contain only uppercase letters and underscores.",
            ),
            PrivilegeApiError::EmptyDescription => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Description cannot be empty.",
            ),
            PrivilegeApiError::DuplicateName => (
                axum::http::StatusCode::CONFLICT,
                "duplicate_privilege_name",
                "Privilege with this name already exists.",
            ),
            PrivilegeApiError::InvalidBody => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Invalid request body.",
            ),
            PrivilegeApiError::InternalError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "Internal server error.",
            ),
        };

        let body = axum::Json(
            serde_json::json!({ "error": error_code, "error_description": error_message }),
        );
        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;

    #[tokio::test]
    async fn test_privilege_api_error_into_response() {
        // One of the errors, to test the response conversion

        let response = PrivilegeApiError::EmptyName.into_response();

        let axum_response = response.into_response();

        assert_eq!(axum_response.status(), StatusCode::BAD_REQUEST);

        let body_bytes = axum::body::to_bytes(axum_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert_eq!(
            body_text,
            "{\"error\":\"invalid_request\",\"error_description\":\"Name cannot be empty.\"}"
        );
    }
}
