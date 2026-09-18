use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The authorization request.
///
/// We store every value in a `Option` because the request may be incomplete or malformed.
/// This allows us to validate the params in order to provide more specific error messages to the client
/// and differentiate between errors that can be redirected to the client and those that cannot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    client_id: Uuid,

    redirect_uri: Option<String>,
    response_type: Option<String>,
    scope: Option<String>,

    state: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

impl AuthorizationRequest {
    pub fn new(
        client_id: Uuid,
        redirect_uri: Option<String>,
        response_type: Option<String>,
        scope: Option<String>,
        state: Option<String>,
        code_challenge: Option<String>,
        code_challenge_method: Option<String>,
    ) -> Self {
        Self {
            client_id,
            redirect_uri,
            response_type,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        }
    }

    pub fn client_id(&self) -> &Uuid {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uri.as_deref()
    }

    pub fn response_type(&self) -> Option<&str> {
        self.response_type.as_deref()
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    pub fn code_challenge(&self) -> Option<&str> {
        self.code_challenge.as_deref()
    }

    pub fn code_challenge_method(&self) -> Option<&str> {
        self.code_challenge_method.as_deref()
    }
}
