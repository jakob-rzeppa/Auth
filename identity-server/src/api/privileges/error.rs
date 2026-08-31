use axum::response::IntoResponse;

pub enum PrivilegeApiError {}

impl IntoResponse for PrivilegeApiError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
