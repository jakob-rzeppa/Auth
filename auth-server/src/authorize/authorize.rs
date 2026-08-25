use axum::extract::State;

use crate::{
    app_state::AppState,
    authorize::{
        error::{
            AuthorizeError,
            fatal::{FatalAuthorizeError, FatalAuthorizeErrorKind},
        },
        request::AuthorizeRequest,
    },
    persistence::{PersistanceError, client::get_client_by_id, user::get_user_by_email},
    service::client::{validate_redirect_uri, validate_scope},
};

#[axum::debug_handler]
pub async fn authorize_endpoint(
    State(app_state): State<AppState>,
    request: AuthorizeRequest,
) -> Result<(), AuthorizeError> {
    let AuthorizeRequest {
        response_type,
        client_id,
        redirect_uri,
        scope,
        state,
        user_email,
        user_password,
    } = request;

    // ==== Validate Client ====
    let client = get_client_by_id(&client_id).map_err(|e| match e {
        PersistanceError::DatabaseError { .. } => AuthorizeError::Fatal(FatalAuthorizeError::new(
            FatalAuthorizeErrorKind::ServerError,
        )),
    })?;
    let Some(client) = client else {
        return Err(AuthorizeError::Fatal(FatalAuthorizeError::new(
            FatalAuthorizeErrorKind::UnauthorizedClient,
        )));
    };

    if !validate_redirect_uri(&client, &redirect_uri) {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new_invalid_request(
                "The redirect_uri is not registered with the client.",
            ),
        ));
    }
    // From here on we can safely use the redirect_uri for error responses, since it is validated.
    // See https://oauth.net/advisories/2014-1-covert-redirect/.

    if !validate_scope(&client, &scope) {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new(FatalAuthorizeErrorKind::InvalidScope)
                .with_redirect_uri(redirect_uri),
        ));
    }

    if response_type != "code" {
        return Err(AuthorizeError::Fatal(
            FatalAuthorizeError::new(FatalAuthorizeErrorKind::UnsupportedResponseType)
                .with_redirect_uri(redirect_uri),
        ));
    }

    // ==== Authenticate User ====
    let user = get_user_by_email(&user_email);

    Ok(())
}
