use axum::response::IntoResponse;

pub mod fatal;
pub mod recoverable;

pub enum AuthorizeError {
    Fatal(fatal::FatalAuthorizeError),
    Recoverable(recoverable::RecoverableAuthorizeError),
}

impl IntoResponse for AuthorizeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthorizeError::Fatal(err) => err.into_response(),
            AuthorizeError::Recoverable(err) => err.into_response(),
        }
    }
}
