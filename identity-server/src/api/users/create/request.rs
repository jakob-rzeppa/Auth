use api_macros::ApiRequest;

use crate::api::users::create::error_response::CreateUserErrorResponse;

#[ApiRequest(CreateUserErrorResponse::InvalidBody)]
pub struct CreateUserRequest {
    pub email: String,
}
