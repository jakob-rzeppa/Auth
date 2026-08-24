use axum::extract::State;

use crate::app_state::AppState;

#[axum::debug_handler]
pub async fn authorize_endpoint(State(state): State<AppState>) {}
