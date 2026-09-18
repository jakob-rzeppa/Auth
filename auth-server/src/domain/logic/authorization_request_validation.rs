use crate::domain::entity::{authorization_request::AuthorizationRequest, client::Client};

#[derive(Debug, PartialEq)]
pub enum FatalAuthorizationError {
    ClientIdMismatch,
    InvalidRedirectUri,
}

#[derive(Debug, PartialEq)]
pub enum RedirectableAuthorizationError {
    InvalidResponseType,
    InvalidScope,
    InvalidState,
    InvalidCodeChallengeMethod,
    InvalidCodeChallenge,
}

#[derive(Debug, PartialEq)]
pub enum AuthorizationError {
    Fatal(FatalAuthorizationError),
    Redirectable(RedirectableAuthorizationError),
}

impl AuthorizationError {
    pub fn fatal(err: FatalAuthorizationError) -> Self {
        AuthorizationError::Fatal(err)
    }

    pub fn redirectable(err: RedirectableAuthorizationError) -> Self {
        AuthorizationError::Redirectable(err)
    }
}

pub fn validate_authorization_request_against_client(
    request: &AuthorizationRequest,
    client: &Client,
) -> Result<(), AuthorizationError> {
    use FatalAuthorizationError as FatalError;
    use RedirectableAuthorizationError as RedirectableError;

    // ==== client id ====
    if request.client_id() != client.id() {
        return Err(AuthorizationError::fatal(FatalError::ClientIdMismatch));
    }

    // ==== redirect_uri ====
    let Some(redirect_uri) = request.redirect_uri() else {
        return Err(AuthorizationError::fatal(FatalError::InvalidRedirectUri));
    };
    if !client.has_redirect_uri(redirect_uri) {
        return Err(AuthorizationError::fatal(FatalError::InvalidRedirectUri));
    }

    // Only allow error redirects from here on, since the client_id and redirect_uri are valid.

    // ==== response type ====
    let Some(response_type) = request.response_type() else {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidResponseType,
        ));
    };
    if response_type != "code" {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidResponseType,
        ));
    }

    // ==== scope ====
    let Some(scopes) = request.scopes() else {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidScope,
        ));
    };
    if !client.has_scopes(scopes) {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidScope,
        ));
    }

    // ==== state ====
    let Some(state) = request.state() else {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidState,
        ));
    };
    if state.is_empty() {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidState,
        ));
    }

    // ==== code_challenge_method ====
    let Some(code_challenge_method) = request.code_challenge_method() else {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidCodeChallengeMethod,
        ));
    };
    if code_challenge_method != "S256" {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidCodeChallengeMethod,
        ));
    }

    // === code_challenge ====
    let Some(code_challenge) = request.code_challenge() else {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidCodeChallenge,
        ));
    };
    if code_challenge.is_empty() {
        return Err(AuthorizationError::redirectable(
            RedirectableError::InvalidCodeChallenge,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_client(id: Uuid) -> Client {
        Client::new(
            id,
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec!["read".to_string(), "write".to_string()],
        )
    }

    fn make_authorization_request(client_id: Uuid) -> AuthorizationRequest {
        AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        )
    }

    #[test]
    fn valid_par_passes_validation() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = make_authorization_request(client_id);

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Ok(())
        );
    }

    #[test]
    fn rejects_client_id_mismatch() {
        let client = make_client(Uuid::new_v4());
        let par = make_authorization_request(Uuid::new_v4());

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::fatal(
                FatalAuthorizationError::ClientIdMismatch
            ))
        );
    }

    #[test]
    fn rejects_invalid_response_type() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("token".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidResponseType
            ))
        );
    }

    #[test]
    fn rejects_missing_response_type() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            None,
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidResponseType
            ))
        );
    }

    #[test]
    fn rejects_missing_scopes() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            None,
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidScope
            ))
        );
    }

    #[test]
    fn rejects_missing_state() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            None,
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidState
            ))
        );
    }

    #[test]
    fn rejects_missing_code_challenge_method() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            None,
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidCodeChallengeMethod
            ))
        );
    }

    #[test]
    fn rejects_missing_code_challenge() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            None,
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidCodeChallenge
            ))
        );
    }

    #[test]
    fn rejects_missing_redirect_uri() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            None,
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::fatal(
                FatalAuthorizationError::InvalidRedirectUri
            ))
        );
    }

    #[test]
    fn rejects_unregistered_redirect_uri() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://evil.example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::fatal(
                FatalAuthorizationError::InvalidRedirectUri
            ))
        );
    }

    #[test]
    fn rejects_unregistered_scope() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["delete".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidScope
            ))
        );
    }

    #[test]
    fn rejects_empty_state() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidState
            ))
        );
    }

    #[test]
    fn rejects_non_s256_code_challenge_method() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("plain".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidCodeChallengeMethod
            ))
        );
    }

    #[test]
    fn rejects_empty_code_challenge() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let par = AuthorizationRequest::new(
            client_id,
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some(vec!["read".to_string()]),
            Some("some-state".to_string()),
            Some("".to_string()),
            Some("S256".to_string()),
        );

        assert_eq!(
            validate_authorization_request_against_client(&par, &client),
            Err(AuthorizationError::redirectable(
                RedirectableAuthorizationError::InvalidCodeChallenge
            ))
        );
    }
}
