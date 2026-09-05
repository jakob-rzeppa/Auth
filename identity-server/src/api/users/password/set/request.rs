use api_macros::ApiRequest;

use crate::api::users::password::set::error_response::SetPasswordErrorResponse;

#[ApiRequest(SetPasswordErrorResponse::InvalidBody)]
#[derive(utoipa::ToSchema)]
pub struct SetPasswordRequest {
    pub new_password: String,
}
