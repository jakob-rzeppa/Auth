use api_macros::ApiRequest;

use crate::api::users::create::error_response::CreateUserErrorResponse;

#[ApiRequest(CreateUserErrorResponse::InvalidBody)]
#[derive(utoipa::ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
}
