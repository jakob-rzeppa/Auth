use serde::{Deserialize, Serialize};

/// A pushed authorization request to be handled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushedAuthorizationRequest {
    /// A cryptographically strong identifier for the request, the client passes in the /authorize request.
    request_uri: String,

    client_id: String,

    response_type: String,
    scope: String,

    state: String,
    code_challenge: String,
    code_challenge_method: String,
}

impl PushedAuthorizationRequest {
    pub fn new(
        request_uri: String,
        client_id: String,
        response_type: String,
        scope: String,
        state: String,
        code_challenge: String,
        code_challenge_method: String,
    ) -> Self {
        Self {
            request_uri,
            client_id,
            response_type,
            scope,
            state,
            code_challenge,
            code_challenge_method,
        }
    }

    pub fn request_uri(&self) -> &str {
        &self.request_uri
    }
}
