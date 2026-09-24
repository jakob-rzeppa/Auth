use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use uuid::Uuid;

use crate::{
    application::authorization_code::error::{
        AuthCodeError, FatalAuthCodeError, RedirectableAuthCodeError,
    },
    domain::entity::authorization_code::{
        code::AuthorizationCode, request::validate::ValidatedAuthorizationRequest,
    },
    persistence::{
        authorization_codes::save::save_authorization_code, clients::find_by_id::find_client_by_id,
        pars::take::take_par,
    },
};

#[derive(Debug, PartialEq)]
pub struct AuthorizeCodeSuccess {
    pub code: String,
    pub redirect_uri: String,
    pub state: String,
    pub expires_in: u64,
}

const CODE_TTL_SECONDS: u64 = 300; // 5 minutes

pub async fn validate_and_generate_code(
    client_id: Uuid,
    request_uri: String,
) -> Result<AuthorizeCodeSuccess, AuthCodeError> {
    let request = take_par(&request_uri)
        .await
        .map_err(|_| AuthCodeError::fatal(FatalAuthCodeError::DatabaseError))?;
    let Some(request) = request else {
        return Err(AuthCodeError::fatal(
            FatalAuthCodeError::PushedRequestNotFound,
        ));
    };

    let client = find_client_by_id(&client_id)
        .ok_or(AuthCodeError::fatal(FatalAuthCodeError::ClientNotFound))?;

    let ValidatedAuthorizationRequest {
        client_id,
        redirect_uri,
        response_type: _,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    } = request
        .validate_and_unpack(&client)
        .map_err(|e| AuthCodeError::from(e))?;

    // With the request validated, we can now generate an authorization code and return it to the client.

    let code_key = generate_auth_code();

    let code = AuthorizationCode::new(
        code_key.clone(),
        client_id,
        redirect_uri.clone(),
        scope,
        code_challenge,
        code_challenge_method,
    );

    let Ok(()) = save_authorization_code(code, CODE_TTL_SECONDS).await else {
        return Err(AuthCodeError::Redirectable {
            error: RedirectableAuthCodeError::DatabaseError,
            redirect_uri,
            state,
        });
    };

    Ok(AuthorizeCodeSuccess {
        code: code_key,
        redirect_uri,
        state,
        expires_in: CODE_TTL_SECONDS,
    })
}

/// Generate a random 256-bit authorization code and encode it in URL-safe base64.
#[fnmock::fakeable]
fn generate_auth_code() -> String {
    let mut bytes = [0u8; 32]; // 256 bits
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entity::{authorization_code::request::AuthorizationRequest, client::Client},
        persistence::{
            authorization_codes::save::{SaveAuthorizationCodeError, save_authorization_code_mock},
            clients::find_by_id::find_client_by_id_fake,
            pars::take::{TakeParError, take_par_fake},
        },
    };

    fn make_client(id: Uuid) -> Client {
        Client::new(
            id,
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec!["read".to_string(), "write".to_string()],
        )
    }

    fn make_request(client_id: Uuid) -> AuthorizationRequest {
        AuthorizationRequest::new(
            client_id,
            "https://example.com/callback".to_string(),
            "code".to_string(),
            "read write".to_string(),
            "some-state".to_string(),
            "some-code-challenge".to_string(),
            "S256".to_string(),
        )
    }

    #[tokio::test]
    async fn succeeds_and_returns_code_with_ttl_for_a_valid_request() {
        let client_id = Uuid::new_v4();
        let request_uri = "urn:authorize:request_uri:test".to_string();
        let client = make_client(client_id);
        let request = make_request(client_id);

        let expected_request_uri = request_uri.clone();
        take_par_fake().setup(move |uri| {
            assert_eq!(uri, expected_request_uri.as_str());
            Ok(Some(request.clone()))
        });
        find_client_by_id_fake().setup(move |_| Some(client.clone()));
        generate_auth_code_fake().setup(|| "test-code".to_string());
        let mock = save_authorization_code_mock();
        mock.setup(|_, _| Ok(()));
        mock.expectf(move |code: &AuthorizationCode, ttl_seconds: &u64| {
            code.code() == "test-code"
                && code.client_id() == &client_id
                && code.redirect_uri() == "https://example.com/callback"
                && code.scope() == "read write"
                && code.code_challenge() == "some-code-challenge"
                && code.code_challenge_method() == "S256"
                && *ttl_seconds == CODE_TTL_SECONDS
        })
        .once();

        let result = validate_and_generate_code(client_id, request_uri).await;

        let success = result.expect("expected a successful result");
        assert_eq!(success.code, "test-code");
        assert_eq!(success.redirect_uri, "https://example.com/callback");
        assert_eq!(success.state, "some-state");
        assert_eq!(success.expires_in, CODE_TTL_SECONDS);

        mock.assert();
    }

    #[tokio::test]
    async fn fails_with_database_error_when_take_par_fails() {
        let client_id = Uuid::new_v4();

        take_par_fake().setup(|_| Err(TakeParError::DatabaseError));

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:test".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::fatal(FatalAuthCodeError::DatabaseError))
        );
    }

    #[tokio::test]
    async fn fails_when_pushed_request_is_not_found() {
        let client_id = Uuid::new_v4();

        take_par_fake().setup(|_| Ok(None));

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:missing".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::fatal(
                FatalAuthCodeError::PushedRequestNotFound
            ))
        );
    }

    #[tokio::test]
    async fn fails_when_client_is_not_found() {
        let client_id = Uuid::new_v4();
        let request = make_request(client_id);

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(|_| None);

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:test".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::fatal(FatalAuthCodeError::ClientNotFound))
        );
    }

    #[tokio::test]
    async fn fails_with_fatal_error_when_redirect_uri_is_not_registered() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = AuthorizationRequest::new(
            client_id,
            "https://evil.example.com/callback".to_string(),
            "code".to_string(),
            "read write".to_string(),
            "some-state".to_string(),
            "some-code-challenge".to_string(),
            "S256".to_string(),
        );

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:test".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::fatal(FatalAuthCodeError::InvalidRedirectUri))
        );
    }

    #[tokio::test]
    async fn fails_with_redirectable_error_when_scope_is_not_allowed() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = AuthorizationRequest::new(
            client_id,
            "https://example.com/callback".to_string(),
            "code".to_string(),
            "delete".to_string(),
            "some-state".to_string(),
            "some-code-challenge".to_string(),
            "S256".to_string(),
        );

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:test".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::redirectable(
                RedirectableAuthCodeError::InvalidScope,
                "https://example.com/callback".to_string(),
                "some-state".to_string(),
            ))
        );
    }

    #[tokio::test]
    async fn fails_with_redirectable_database_error_when_save_authorization_code_fails() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = make_request(client_id);

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));
        generate_auth_code_fake().setup(|| "test-code".to_string());
        save_authorization_code_mock().setup(|_, _| Err(SaveAuthorizationCodeError::DatabaseError));

        let result =
            validate_and_generate_code(client_id, "urn:authorize:request_uri:test".to_string())
                .await;

        assert_eq!(
            result,
            Err(AuthCodeError::redirectable(
                RedirectableAuthCodeError::DatabaseError,
                "https://example.com/callback".to_string(),
                "some-state".to_string(),
            ))
        );
    }
}
