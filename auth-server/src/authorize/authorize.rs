use crate::{
    authorize::{
        error::{
            AuthorizeError,
            fatal::{FatalAuthorizeError, FatalAuthorizeErrorKind},
            recoverable::RecoverableAuthorizeError,
        },
        request::AuthorizeRequest,
    },
    persistence::{PersistanceError, client::get_client_by_id, user::get_user_by_email},
    service::{
        client::{validate_redirect_uri, validate_scope},
        user::{UserHandlerError, check_password_authentication},
    },
};

#[axum::debug_handler]
pub async fn authorize_endpoint(request: AuthorizeRequest) -> Result<(), AuthorizeError> {
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
    let user = get_user_by_email(&user_email).map_err(|e| match e {
        PersistanceError::DatabaseError { .. } => AuthorizeError::Fatal(FatalAuthorizeError::new(
            FatalAuthorizeErrorKind::ServerError,
        )),
    })?;
    let Some(user) = user else {
        return Err(AuthorizeError::Recoverable(RecoverableAuthorizeError {
            kind: crate::authorize::error::recoverable::RecoverableAuthorizeErrorKind::InvalidCredentials,
            client_id: client_id.to_string(),
            redirect_uri,
            scope: scope.join(" "),
            state,
            user_name: user_email,
        }));
    };

    check_password_authentication(&user, &user_password).map_err(|e| match e {
        UserHandlerError::InvalidPassword => {
            AuthorizeError::Recoverable(RecoverableAuthorizeError {
                kind: crate::authorize::error::recoverable::RecoverableAuthorizeErrorKind::InvalidCredentials,
                client_id: client_id.to_string(),
                redirect_uri,
                scope: scope.join(" "),
                state,
                user_name: user_email,
            })
        }
    })?;

    Ok(())
}
