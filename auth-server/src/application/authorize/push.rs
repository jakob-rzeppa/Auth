use uuid::Uuid;

use crate::{
    domain::{
        entity::authorization_request::AuthorizationRequest,
        logic::authorization_request_validation::{
            AuthorizationError, FatalAuthorizationError, RedirectableAuthorizationError,
            validate_authorization_request_against_client,
        },
    },
    persistence::{clients::find_by_id::find_client_by_id, pars::save::save_par},
};

#[derive(Debug, PartialEq)]
pub enum FatalAuthorizePushError {
    ClientNotFound,
    InvalidRedirectUri,
}

#[derive(Debug, PartialEq)]
pub enum RedirectableAuthorizePushError {
    InvalidResponseType,
    InvalidScope,
    InvalidState,
    InvalidCodeChallengeMethod,
    InvalidCodeChallenge,
    DatabaseError,
}

#[derive(Debug, PartialEq)]
pub enum AuthorizePushError {
    Fatal(FatalAuthorizePushError),
    Redirectable(RedirectableAuthorizePushError),
}

#[derive(Debug, PartialEq)]
pub struct AuthorizePushSuccess {
    pub request_uri: String,
    pub expires_in: u64, // seconds
}

const PAR_TTL_SECONDS: u64 = 180; // 3 minutes

pub async fn validate_and_register_authorize_push(
    client_id: Uuid,

    redirect_uri: Option<String>,
    response_type: Option<String>,
    scope: Option<String>, // space delimited list of scopes

    state: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
) -> Result<AuthorizePushSuccess, AuthorizePushError> {
    let client = find_client_by_id(&client_id).ok_or(AuthorizePushError::Fatal(
        FatalAuthorizePushError::ClientNotFound,
    ))?;

    let scopes: Option<Vec<String>> =
        scope.map(|s| s.split_whitespace().map(|s| s.to_string()).collect());

    let request = AuthorizationRequest::new(
        client_id,
        redirect_uri,
        response_type,
        scopes,
        state,
        code_challenge,
        code_challenge_method,
    );

    validate_authorization_request_against_client(&request, &client).map_err(|err| match err {
        AuthorizationError::Fatal(fatal) => AuthorizePushError::Fatal(match fatal {
            FatalAuthorizationError::InvalidRedirectUri => FatalAuthorizePushError::InvalidRedirectUri,
            FatalAuthorizationError::ClientIdMismatch => unreachable!(
                "Client ID mismatch should not occur here as we already fetched the client by the same id."
            )
        }),
        AuthorizationError::Redirectable(redirectable) => AuthorizePushError::Redirectable(match redirectable {
            RedirectableAuthorizationError::InvalidCodeChallenge => RedirectableAuthorizePushError::InvalidCodeChallenge,
            RedirectableAuthorizationError::InvalidCodeChallengeMethod => RedirectableAuthorizePushError::InvalidCodeChallengeMethod,
            RedirectableAuthorizationError::InvalidResponseType => RedirectableAuthorizePushError::InvalidResponseType,
            RedirectableAuthorizationError::InvalidScope => RedirectableAuthorizePushError::InvalidScope,
            RedirectableAuthorizationError::InvalidState => RedirectableAuthorizePushError::InvalidState,
        }),
    })?;

    let request_uri = generate_request_uri();

    save_par(&request_uri, request, PAR_TTL_SECONDS)
        .await
        .map_err(|_| {
            AuthorizePushError::Redirectable(RedirectableAuthorizePushError::DatabaseError)
        })?;

    Ok(AuthorizePushSuccess {
        request_uri,
        expires_in: PAR_TTL_SECONDS,
    })
}

const REQUEST_URI_PREFIX: &str = "urn:authorize:request_uri:";

/// Generate a unique request URI for the pushed authorization request.
///
/// Per RFC 9126, the `request_uri` must be hard to guess (a "https" or "urn" scheme
/// value containing a cryptographically random component). We use a UUIDv4, which is
/// generated from a CSPRNG, appended to a URN prefix.
#[fnmock::fakeable]
fn generate_request_uri() -> String {
    format!("{REQUEST_URI_PREFIX}{}", Uuid::new_v4())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::entity::client::Client,
        persistence::{
            clients::find_by_id::find_client_by_id_fake, pars::save::SaveParError,
            pars::save::save_par_fake,
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

    /// A fully valid set of request params (everything but `client_id`), in the same order as
    /// `validate_and_register_authorize_push`'s parameters.
    fn valid_params() -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        (
            Some("https://example.com/callback".to_string()),
            Some("code".to_string()),
            Some("read write".to_string()),
            Some("some-state".to_string()),
            Some("some-code-challenge".to_string()),
            Some("S256".to_string()),
        )
    }

    #[tokio::test]
    async fn succeeds_and_returns_request_uri_with_ttl_for_a_valid_request() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);

        find_client_by_id_fake().setup(move |_| Some(client.clone()));
        save_par_fake().setup(move |request_uri, par, ttl_seconds| {
            assert!(request_uri.starts_with(REQUEST_URI_PREFIX));
            assert_eq!(ttl_seconds, PAR_TTL_SECONDS);
            assert_eq!(par.client_id(), &client_id);
            assert_eq!(par.redirect_uri(), Some("https://example.com/callback"));
            assert_eq!(par.state(), Some("some-state"));
            Ok(())
        });
        generate_request_uri_fake().setup(|| "urn:authorize:request_uri:test".to_string());

        let (redirect_uri, response_type, scope, state, code_challenge, code_challenge_method) =
            valid_params();
        let result = validate_and_register_authorize_push(
            client_id,
            redirect_uri,
            response_type,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        )
        .await;

        let success = result.expect("expected a successful result");
        assert_eq!(success.request_uri, "urn:authorize:request_uri:test");
        assert_eq!(success.expires_in, PAR_TTL_SECONDS);
    }

    #[tokio::test]
    async fn fails_with_database_error_when_save_par_fails() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);

        find_client_by_id_fake().setup(move |_| Some(client.clone()));
        save_par_fake().setup(|_, _, _| Err(SaveParError::DatabaseError));
        generate_request_uri_fake().setup(|| "urn:authorize:request_uri:test".to_string());

        let (redirect_uri, response_type, scope, state, code_challenge, code_challenge_method) =
            valid_params();
        let result = validate_and_register_authorize_push(
            client_id,
            redirect_uri,
            response_type,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        )
        .await;

        assert_eq!(
            result,
            Err(AuthorizePushError::Redirectable(
                RedirectableAuthorizePushError::DatabaseError
            ))
        );
    }
}
