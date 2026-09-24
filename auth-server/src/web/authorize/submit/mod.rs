use crate::{
    application::authorization_code::submit::{AuthorizeCodeSuccess, validate_and_generate_code},
    web::authorize::submit::{
        error_response::AuthorizeSubmitErrorResponse, request::AuthorizeSubmitRequest,
        response::AuthorizeSubmitResponse,
    },
};

pub mod error_response;
pub mod request;
pub mod response;

#[axum::debug_handler]
pub async fn authorize_submit_endpoint(
    AuthorizeSubmitRequest {
        request_uri,
        client_id,
    }: AuthorizeSubmitRequest,
) -> Result<AuthorizeSubmitResponse, AuthorizeSubmitErrorResponse> {
    let AuthorizeCodeSuccess {
        code,
        redirect_uri,
        expires_in,
        state,
    } = validate_and_generate_code(client_id, request_uri)
        .await
        .map_err(AuthorizeSubmitErrorResponse::from)?;

    Ok(AuthorizeSubmitResponse {
        code,
        redirect_uri,
        state,
        expires_in,
    })
}
