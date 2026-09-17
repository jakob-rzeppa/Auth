use serde::{Deserialize, Serialize};

/// The authorization request.
///
/// We store every value in a `Option` because the request may be incomplete or malformed.
/// This allows us to validate the params in order to provide more specific error messages to the client
/// and differentiate between errors that can be redirected to the client and those that cannot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    client_id: Option<String>,

    redirect_uri: Option<String>,
    response_type: Option<String>,
    scopes: Option<Vec<String>>,

    state: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

impl AuthorizationRequest {
    pub fn new(
        client_id: Option<String>,
        redirect_uri: Option<String>,
        response_type: Option<String>,
        scopes: Option<Vec<String>>,
        state: Option<String>,
        code_challenge: Option<String>,
        code_challenge_method: Option<String>,
    ) -> Self {
        Self {
            client_id,
            redirect_uri,
            response_type,
            scopes,
            state,
            code_challenge,
            code_challenge_method,
        }
    }

    pub fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }

    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uri.as_deref()
    }

    pub fn response_type(&self) -> Option<&str> {
        self.response_type.as_deref()
    }

    pub fn scopes(&self) -> Option<&[String]> {
        self.scopes.as_deref()
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
