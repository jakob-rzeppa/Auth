use uuid::Uuid;

use crate::{
    application::authorization_code::error::{
        AuthCodeError, FatalAuthCodeError, RedirectableAuthCodeError,
    },
    domain::entity::authorization_code::request::AuthorizationRequest,
    persistence::{clients::find_by_id::find_client_by_id, pars::save::save_par},
};

#[derive(Debug, PartialEq)]
pub struct AuthorizePushSuccess {
    pub request_uri: String,
    pub expires_in: u64, // seconds
}

const PAR_TTL_SECONDS: u64 = 180; // 3 minutes

pub async fn validate_and_register_authorize_push(
    client_id: Uuid,

    redirect_uri: String,
    response_type: String,
    scope: String, // space delimited list of scopes

    state: String,
    code_challenge: String,
    code_challenge_method: String,
) -> Result<AuthorizePushSuccess, AuthCodeError> {
    let client = find_client_by_id(&client_id)
        .ok_or(AuthCodeError::fatal(FatalAuthCodeError::ClientNotFound))?;

    let request = AuthorizationRequest::new(
        client_id,
        redirect_uri,
        response_type,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    );

    request.validate_against_client(&client)?;

    let request_uri = generate_request_uri();

    let redirect_uri = request.redirect_uri().to_string();
    let state = request.state().to_string();

    save_par(&request_uri, request, PAR_TTL_SECONDS)
        .await
        .map_err(|_| AuthCodeError::Redirectable {
            error: RedirectableAuthCodeError::DatabaseError,
            redirect_uri,
            state,
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
    fn valid_params() -> (String, String, String, String, String, String) {
        (
            "https://example.com/callback".to_string(),
            "code".to_string(),
            "read write".to_string(),
            "some-state".to_string(),
            "some-code-challenge".to_string(),
            "S256".to_string(),
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
            assert_eq!(par.redirect_uri(), "https://example.com/callback");
            assert_eq!(par.state(), "some-state");
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
            redirect_uri.clone(),
            response_type,
            scope,
            state.clone(),
            code_challenge,
            code_challenge_method,
        )
        .await;

        assert_eq!(
            result,
            Err(AuthCodeError::Redirectable {
                error: RedirectableAuthCodeError::DatabaseError,
                redirect_uri,
                state
            })
        );
    }
}
