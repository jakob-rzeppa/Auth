use axum::{extract::Path, response::Response};

use crate::{
    api::users::update::{error_response::UpdateUserErrorResponse, request::UpdateUserRequest},
    application::users::update::{UpdateUserError, update_user},
};

pub mod error_response;
pub mod request;

pub async fn update_user_endpoint(
    Path(user_id): Path<String>,
    UpdateUserRequest { email }: UpdateUserRequest,
) -> Result<Response, UpdateUserErrorResponse> {
    let Ok(user_id) = uuid::Uuid::parse_str(&user_id) else {
        return Err(UpdateUserErrorResponse::InvalidUserId);
    };

    update_user(user_id, email).await.map_err(|err| match err {
        UpdateUserError::UserNotFound => UpdateUserErrorResponse::UserNotFound,
        UpdateUserError::InvalidEmailFormat => UpdateUserErrorResponse::InvalidEmail,
        UpdateUserError::EmailAlreadyExists => UpdateUserErrorResponse::EmailAlreadyExists,
        UpdateUserError::DatabaseError => UpdateUserErrorResponse::InternalServerError,
    })?;

    Ok(Response::builder()
        .status(204)
        .body(axum::body::Body::empty())
        .unwrap())
}
