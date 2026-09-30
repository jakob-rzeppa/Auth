use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;

use crate::{
    domain::entity::authorization_code::{
        code::AuthorizationCode, request::validate::ValidatedAuthorizationRequest,
    },
    persistence::{
        authorization_codes::save::save_authorization_code, clients::find_by_id::find_client_by_id,
        pars::take::take_par,
    },
    web::authorize::submit::{
        error_response::{
            AuthorizeSubmitErrorResponse, AuthorizeSubmitPageErrorResponse,
            AuthorizeSubmitRedirectErrorResponse,
        },
        request::AuthorizeSubmitRequest,
        response::AuthorizeSubmitResponse,
    },
};

pub mod error_response;
pub mod request;
pub mod response;

const CODE_TTL_SECONDS: u64 = 300; // 5 minutes

#[axum::debug_handler]
pub async fn authorize_submit_endpoint(
    AuthorizeSubmitRequest {
        request_uri,
        client_id,
    }: AuthorizeSubmitRequest,
) -> Result<AuthorizeSubmitResponse, AuthorizeSubmitErrorResponse> {
    let page_error = |error| AuthorizeSubmitErrorResponse::Page { error };

    let request = take_par(&request_uri)
        .await
        .map_err(|_| page_error(AuthorizeSubmitPageErrorResponse::ServerError))?
        .ok_or(page_error(
            AuthorizeSubmitPageErrorResponse::RequestNotFound,
        ))?;

    let client = find_client_by_id(&client_id)
        .ok_or(page_error(AuthorizeSubmitPageErrorResponse::ClientNotFound))?;

    let ValidatedAuthorizationRequest {
        client_id,
        redirect_uri,
        scope,
        state,
        code_challenge,
        code_challenge_method,
    } = request
        .validate_and_unpack(&client)
        .map_err(AuthorizeSubmitErrorResponse::from)?;

    // With the request validated, we can now generate an authorization code and return it to the client.

    let code = generate_auth_code();

    let authorization_code = AuthorizationCode::new(
        code.clone(),
        client_id,
        scope,
        code_challenge,
        code_challenge_method,
    );

    save_authorization_code(authorization_code, CODE_TTL_SECONDS)
        .await
        .map_err(|_| AuthorizeSubmitErrorResponse::Redirect {
            error: AuthorizeSubmitRedirectErrorResponse::ServerError,
            redirect_uri: redirect_uri.clone(),
            state: state.clone(),
        })?;

    Ok(AuthorizeSubmitResponse {
        code,
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
    use uuid::Uuid;

    fn make_client(id: Uuid) -> Client {
        Client::new(
            id,
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec!["read".to_string(), "write".to_string()],
        )
    }

    fn make_request_with(client_id: Uuid, redirect_uri: &str, scope: &str) -> AuthorizationRequest {
        AuthorizationRequest::new(
            client_id,
            redirect_uri.to_string(),
            "code".to_string(),
            scope.to_string(),
            "some-state".to_string(),
            "some-code-challenge".to_string(),
            "S256".to_string(),
        )
    }

    fn make_request(client_id: Uuid) -> AuthorizationRequest {
        make_request_with(client_id, "https://example.com/callback", "read write")
    }

    fn submit_request(client_id: Uuid, request_uri: &str) -> AuthorizeSubmitRequest {
        AuthorizeSubmitRequest {
            request_uri: request_uri.to_string(),
            client_id,
        }
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
                && code.scope() == "read write"
                && code.code_challenge() == "some-code-challenge"
                && code.code_challenge_method() == "S256"
                && *ttl_seconds == CODE_TTL_SECONDS
        })
        .once();

        let result = authorize_submit_endpoint(submit_request(client_id, &request_uri)).await;

        let Ok(response) = result else {
            panic!("expected a successful result");
        };
        assert_eq!(response.code, "test-code");
        assert_eq!(response.redirect_uri, "https://example.com/callback");
        assert_eq!(response.state, "some-state");
        assert_eq!(response.expires_in, CODE_TTL_SECONDS);

        mock.assert();
    }

    #[tokio::test]
    async fn fails_with_server_error_when_take_par_fails() {
        take_par_fake().setup(|_| Err(TakeParError::DatabaseError));

        let result = authorize_submit_endpoint(submit_request(
            Uuid::new_v4(),
            "urn:authorize:request_uri:test",
        ))
        .await;

        assert!(matches!(
            result,
            Err(AuthorizeSubmitErrorResponse::Page {
                error: AuthorizeSubmitPageErrorResponse::ServerError
            })
        ));
    }

    #[tokio::test]
    async fn fails_when_pushed_request_is_not_found() {
        take_par_fake().setup(|_| Ok(None));

        let result = authorize_submit_endpoint(submit_request(
            Uuid::new_v4(),
            "urn:authorize:request_uri:missing",
        ))
        .await;

        assert!(matches!(
            result,
            Err(AuthorizeSubmitErrorResponse::Page {
                error: AuthorizeSubmitPageErrorResponse::RequestNotFound
            })
        ));
    }

    #[tokio::test]
    async fn fails_when_client_is_not_found() {
        let client_id = Uuid::new_v4();
        let request = make_request(client_id);

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(|_| None);

        let result =
            authorize_submit_endpoint(submit_request(client_id, "urn:authorize:request_uri:test"))
                .await;

        assert!(matches!(
            result,
            Err(AuthorizeSubmitErrorResponse::Page {
                error: AuthorizeSubmitPageErrorResponse::ClientNotFound
            })
        ));
    }

    #[tokio::test]
    async fn fails_with_page_error_when_redirect_uri_is_not_registered() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = make_request_with(client_id, "https://evil.example.com/callback", "read");

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));

        let result =
            authorize_submit_endpoint(submit_request(client_id, "urn:authorize:request_uri:test"))
                .await;

        assert!(matches!(
            result,
            Err(AuthorizeSubmitErrorResponse::Page {
                error: AuthorizeSubmitPageErrorResponse::InvalidRedirectUri
            })
        ));
    }

    #[tokio::test]
    async fn fails_with_redirect_error_when_scope_is_not_allowed() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = make_request_with(client_id, "https://example.com/callback", "delete");

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));

        let result =
            authorize_submit_endpoint(submit_request(client_id, "urn:authorize:request_uri:test"))
                .await;

        let Err(AuthorizeSubmitErrorResponse::Redirect {
            error: AuthorizeSubmitRedirectErrorResponse::InvalidScope,
            redirect_uri,
            state,
        }) = result
        else {
            panic!("expected an invalid_scope redirect error");
        };
        assert_eq!(redirect_uri, "https://example.com/callback");
        assert_eq!(state, "some-state");
    }

    #[tokio::test]
    async fn fails_with_redirect_server_error_when_save_authorization_code_fails() {
        let client_id = Uuid::new_v4();
        let client = make_client(client_id);
        let request = make_request(client_id);

        take_par_fake().setup(move |_| Ok(Some(request.clone())));
        find_client_by_id_fake().setup(move |_| Some(client.clone()));
        generate_auth_code_fake().setup(|| "test-code".to_string());
        save_authorization_code_mock().setup(|_, _| Err(SaveAuthorizationCodeError::DatabaseError));

        let result =
            authorize_submit_endpoint(submit_request(client_id, "urn:authorize:request_uri:test"))
                .await;

        let Err(AuthorizeSubmitErrorResponse::Redirect {
            error: AuthorizeSubmitRedirectErrorResponse::ServerError,
            redirect_uri,
            state,
        }) = result
        else {
            panic!("expected a server_error redirect error");
        };
        assert_eq!(redirect_uri, "https://example.com/callback");
        assert_eq!(state, "some-state");
    }
}
