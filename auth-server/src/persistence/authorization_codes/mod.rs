pub mod save;
pub mod take;

fn key(request_uri: &str) -> String {
    format!("authorization_code:{request_uri}")
}
