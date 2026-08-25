use crate::domain::client::Client;

pub fn validate_redirect_uri(client: &Client, redirect_uri: &str) -> bool {
    if !client.redirect_uris.contains(&redirect_uri.to_string()) {
        false
    } else {
        true
    }
}

pub fn validate_scope(client: &Client, scope: &[String]) -> bool {
    for scope_value in scope {
        if !client.scopes.contains(&scope_value.to_string()) {
            return false;
        }
    }
    true
}
