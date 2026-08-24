use crate::domain::client::Client;

pub fn validate_authentication_for_client(
    client: &Client,
    request_uri: &str,
    scopes: &[&str],
) -> Result<(), String> {
    // Validate the request URI
    if !client.redirect_uris.contains(&request_uri.to_string()) {
        return Err("Invalid redirect URI".to_string());
    }

    // Validate the requested scopes
    for scope in scopes {
        if !client.scopes.contains(&scope.to_string()) {
            return Err(format!("Invalid scope: {}", scope));
        }
    }

    Ok(())
}
