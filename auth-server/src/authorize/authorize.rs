use axum::extract::State;

use crate::{
    app_state::AppState,
    authorize::{error::AuthorizeError, request::AuthorizeRequest},
};

#[axum::debug_handler]
pub async fn authorize_endpoint(
    State(state): State<AppState>,
    request: AuthorizeRequest,
) -> Result<(), AuthorizeError> {
    todo!()
}
