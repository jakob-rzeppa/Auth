use axum::extract::Path;

use crate::{
    api::users::get::{error_response::GetUserErrorResponse, response::GetUserResponse},
    application::users::get_full::{GetFullUserError, get_full_user_projection},
};

pub mod error_response;
pub mod response;

#[axum::debug_handler]
pub async fn get_user_endpoint(
    Path(user_id): Path<String>,
) -> Result<GetUserResponse, GetUserErrorResponse> {
    let Ok(user_id) = uuid::Uuid::parse_str(&user_id) else {
        return Err(GetUserErrorResponse::InvalidUserId);
    };

    let user_projection = get_full_user_projection(user_id)
        .await
        .map_err(|e| match e {
            GetFullUserError::UserNotFound => GetUserErrorResponse::UserNotFound,
            GetFullUserError::DatabaseError => GetUserErrorResponse::InternalServerError,
        })?;

    Ok(GetUserResponse {
        data: user_projection,
    })
}
