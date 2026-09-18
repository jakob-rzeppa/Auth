use api_macros::ApiRequest;

use crate::web::authorize::push::error_response::AuthorizePushErrorResponse;

#[ApiRequest(AuthorizePushErrorResponse::InvalidRequestBody)]
pub struct AuthorizePushRequest {
    pub client_id: Option<String>,
    pub redirect_uri: Option<String>,
    pub response_type: Option<String>,
    pub scope: Option<String>,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
}
