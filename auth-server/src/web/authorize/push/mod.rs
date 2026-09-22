use crate::{
    application::authorization_code::{
        error::{AuthCodeError, FatalAuthCodeError, RedirectableAuthCodeError},
        push::{AuthorizePushSuccess, validate_and_register_authorize_push},
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
        AuthCodeError::Fatal { error } => match error {
            FatalAuthCodeError::ClientNotFound => AuthorizePushErrorResponse::ClientNotFound,
            FatalAuthCodeError::DatabaseError => AuthorizePushErrorResponse::DatabaseError,
            FatalAuthCodeError::PushedRequestNotFound => AuthorizePushErrorResponse::DatabaseError,
            FatalAuthCodeError::ClientIdMismatch => AuthorizePushErrorResponse::InternalServerError,
            FatalAuthCodeError::InvalidRedirectUri => {
                AuthorizePushErrorResponse::InvalidRedirectUri
            }
            FatalAuthCodeError::InvalidState => AuthorizePushErrorResponse::InvalidState,
        },
        AuthCodeError::Redirectable { error, .. } => match error {
            RedirectableAuthCodeError::InvalidResponseType => {
                AuthorizePushErrorResponse::InvalidResponseType
            }
            RedirectableAuthCodeError::InvalidScope => AuthorizePushErrorResponse::InvalidScope,
            RedirectableAuthCodeError::InvalidCodeChallengeMethod => {
                AuthorizePushErrorResponse::InvalidCodeChallengeMethod
            }
            RedirectableAuthCodeError::InvalidCodeChallenge => {
                AuthorizePushErrorResponse::InvalidCodeChallenge
            }
            RedirectableAuthCodeError::DatabaseError => AuthorizePushErrorResponse::DatabaseError,
        },
    })?;

    Ok(AuthorizePushResponse {
        request_uri,
        expires_in,
    })
}
