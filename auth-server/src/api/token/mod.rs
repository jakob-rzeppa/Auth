use base64::Engine;
use chrono::Utc;
use sha2::Digest;

use crate::{
    api::token::{
        error_response::TokenErrorResponse, request::TokenRequest, response::TokenResponse,
    },
    persistence::access_tokens::register::register_access_token,
};

mod error_response;
mod generation;
mod request;
mod response;

mod grant {
    pub mod authorization_code;
}

#[axum::debug_handler]
pub async fn token_endpoint(request: TokenRequest) -> Result<TokenResponse, TokenErrorResponse> {
    let (token, access_token_entity) = match request.grant_type.as_str() {
        "authorization_code" => {
            grant::authorization_code::handle_authorization_code_grant(request).await?
        }
        _ => return Err(TokenErrorResponse::UnsupportedGrantType),
    };

    register_access_token(&access_token_entity)
        .await
        .map_err(|_| TokenErrorResponse::DatabaseError)?;

    Ok(TokenResponse {
        access_token: token,
        token_type: "bearer".to_string(),
        expires_in: access_token_entity.exp().timestamp() - Utc::now().timestamp(),
        scope: access_token_entity.scope().to_string(),
    })
}

/// Verify the code_verifier against the code_challenge.
///
/// # code_challenge_method
///
/// "S256": S256 hashing and Base64URL encoding as per the PKCE specification.
fn verify_code_challenge(
    code_challenge_method: &str,
    code_verifier: &str,
    code_challenge: &str,
) -> bool {
    // Only support S256 method
    if code_challenge_method != "S256" {
        return false;
    }

    // 1. Hash the code_verifier using SHA256
    let hash = sha2::Sha256::digest(code_verifier.as_bytes());

    // 2. Base64URL encode the hash
    let encoded_hash = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);

    // 3. Compare the result with the code_challenge
    encoded_hash == code_challenge
}
