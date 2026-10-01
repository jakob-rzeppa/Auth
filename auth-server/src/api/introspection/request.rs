use api_macros::ApiRequest;

use crate::api::introspection::error_response::IntrospectionErrorResponse;

#[ApiRequest(IntrospectionErrorResponse::InvalidRequestBody)]
pub struct IntrospectionRequest {
    pub token: String,
}
