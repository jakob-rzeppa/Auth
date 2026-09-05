use api_macros::ApiRequest;

use crate::api::users::authenticate::error_response::AuthenticateUserErrorResponse;

#[ApiRequest(AuthenticateUserErrorResponse::InvalidBody)]
#[derive(utoipa::ToSchema)]
pub struct AuthenticateUserRequest {
    pub email: String,
    pub password: String,
}
