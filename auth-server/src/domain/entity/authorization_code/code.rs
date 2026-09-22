use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The authorization code entity, derived from the authorization request on validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationCode {
    code: String,
    client_id: Uuid,

    redirect_uri: String,
    scope: String,

    code_challenge: String,
    code_challenge_method: String,
}

impl AuthorizationCode {
    pub fn new(
        code: String,
        client_id: Uuid,
        redirect_uri: String,
        scope: String,
        code_challenge: String,
        code_challenge_method: String,
    ) -> Self {
        Self {
            code,
            client_id,
            redirect_uri,
            scope,
            code_challenge,
            code_challenge_method,
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn client_id(&self) -> &Uuid {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn code_challenge(&self) -> &str {
        &self.code_challenge
    }

    pub fn code_challenge_method(&self) -> &str {
        &self.code_challenge_method
    }
}
