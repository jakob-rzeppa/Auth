use crate::{
    application::authorize::push::{
        AuthorizePushError, AuthorizePushSuccess, FatalAuthorizePushError,
        RedirectableAuthorizePushError, validate_and_register_authorize_push,
    },
    web::authorize::push::{
        error_response::AuthorizePushErrorResponse, request::AuthorizePushRequest,
        response::AuthorizePushResponse,
    },
};

pub mod error_response;
pub mod request;
pub mod response;

pub async fn authorize_push_endpoint(
    AuthorizePushRequest {
        client_id,
        redirect_uri,
        response_type,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    }: AuthorizePushRequest,
) -> Result<AuthorizePushResponse, AuthorizePushErrorResponse> {
    let client_id = client_id.ok_or(AuthorizePushErrorResponse::InvalidClientId)?;
    let Ok(client_id) = uuid::Uuid::parse_str(&client_id) else {
        return Err(AuthorizePushErrorResponse::InvalidClientId);
    };

    let AuthorizePushSuccess {
        request_uri,
        expires_in,
    } = validate_and_register_authorize_push(
        client_id,
        redirect_uri,
        response_type,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    )
    .await
    .map_err(|err| match err {
        AuthorizePushError::Fatal(fatal) => match fatal {
            FatalAuthorizePushError::ClientNotFound => AuthorizePushErrorResponse::ClientNotFound,
            FatalAuthorizePushError::InvalidRedirectUri => {
                AuthorizePushErrorResponse::InvalidRedirectUri
            }
        },
        AuthorizePushError::Redirectable(redirectable) => match redirectable {
            RedirectableAuthorizePushError::InvalidResponseType => {
                AuthorizePushErrorResponse::InvalidResponseType
            }
            RedirectableAuthorizePushError::InvalidScope => {
                AuthorizePushErrorResponse::InvalidScope
            }
            RedirectableAuthorizePushError::InvalidState => {
                AuthorizePushErrorResponse::InvalidState
            }
            RedirectableAuthorizePushError::InvalidCodeChallengeMethod => {
                AuthorizePushErrorResponse::InvalidCodeChallengeMethod
            }
            RedirectableAuthorizePushError::InvalidCodeChallenge => {
                AuthorizePushErrorResponse::InvalidCodeChallenge
            }
            RedirectableAuthorizePushError::DatabaseError => {
                AuthorizePushErrorResponse::DatabaseError
            }
        },
    })?;

    Ok(AuthorizePushResponse {
        request_uri,
        expires_in,
    })
}
