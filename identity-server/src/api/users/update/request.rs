use api_macros::ApiRequest;

use crate::api::users::update::error_response::UpdateUserErrorResponse;

#[ApiRequest(UpdateUserErrorResponse::InvalidBody)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
}
