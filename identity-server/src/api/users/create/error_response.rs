use axum::response::IntoResponse;

pub enum CreateUserErrorResponse {
    InvalidBody,
    InvalidEmail,
    EmailAlreadyExists,
    InternalServerError,
}

impl IntoResponse for CreateUserErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code, error_message) = match self {
            CreateUserErrorResponse::InvalidBody => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_request",
                "Invalid request body.",
            ),
            CreateUserErrorResponse::InvalidEmail => (
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_email",
                "Email cannot be empty.",
            ),
            CreateUserErrorResponse::EmailAlreadyExists => (
                axum::http::StatusCode::CONFLICT,
                "email_already_exists",
                "Email already exists.",
            ),
            CreateUserErrorResponse::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "An internal server error occurred.",
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
    async fn test_create_user_error_response_into_response() {
        // One of the errors, to test the response conversion

        let response = CreateUserErrorResponse::InvalidEmail.into_response();

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
