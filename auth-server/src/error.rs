use axum::response::IntoResponse;
use serde::Serialize;

pub struct HttpError {
    pub http_status_code: u16,
    pub error: String,
    pub error_description: String,
}

impl HttpError {
    pub fn new(http_status_code: u16, error: String, error_description: String) -> Self {
        Self {
            http_status_code,
            error,
            error_description,
        }
    }
}

#[derive(Serialize)]
struct HttpErrorPayload {
    error: String,
    error_description: String,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&HttpErrorPayload {
            error: self.error,
            error_description: self.error_description,
        })
        .expect("Invalid error payload.");

        (
            axum::http::StatusCode::from_u16(self.http_status_code)
                .expect("Invalid HTTP status code."),
            [("Content-Type", "application/json")],
            body,
        )
            .into_response()
    }
}
