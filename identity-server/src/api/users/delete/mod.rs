use axum::{extract::Path, response::Response};

use crate::{
    api::users::delete::error_response::DeleteUserErrorResponse,
    application::users::delete::{DeleteUserError, delete_user},
};

pub mod error_response;

#[axum::debug_handler]
pub async fn delete_user_endpoint(
    Path(user_id): Path<String>,
) -> Result<Response, DeleteUserErrorResponse> {
    let Ok(user_id) = uuid::Uuid::parse_str(&user_id) else {
        return Err(DeleteUserErrorResponse::InvalidUserId);
    };

    delete_user(user_id).await.map_err(|e| match e {
        DeleteUserError::UserNotFound => DeleteUserErrorResponse::UserNotFound,
        DeleteUserError::DatabaseError => DeleteUserErrorResponse::InternalServerError,
    })?;

    Ok(Response::builder()
        .status(204)
        .body(axum::body::Body::empty())
        .unwrap())
}
