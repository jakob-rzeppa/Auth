use axum::response::IntoResponse;

pub struct CreatePrivilegeResponse {
    id: String,
}

impl IntoResponse for CreatePrivilegeResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
