/// Base URLs of the running services under test.
///
/// Loaded from the process environment, `.env.local` and `.env`, in that order of precedence.
#[derive(Debug, Clone)]
pub struct Config {
    identity_server_url: String,
    auth_server_url: String,
    spec_provider_url: String,
}

impl Config {
    /// Loads `.env.local` and `.env` (never overriding variables that are already set), then reads
    /// the service URLs from the environment.
    ///
    /// # Panics
    ///
    /// Panics if one of the variables is not set.
    pub fn load() -> Self {
        let _ = dotenvy::from_filename(concat!(env!("CARGO_MANIFEST_DIR"), "/.env.local"));
        let _ = dotenvy::from_filename(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

        Self::from_lookup(|key| std::env::var(key).ok())
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Self {
        let get = |key: &str| lookup(key).unwrap_or_else(|| panic!("{key} must be set"));

        Self {
            identity_server_url: get("IDENTITY_SERVER_URL"),
            auth_server_url: get("AUTH_SERVER_URL"),
            spec_provider_url: get("SPEC_PROVIDER_URL"),
        }
    }

    pub fn identity_url(&self, path: &str) -> String {
        join(&self.identity_server_url, path)
    }

    pub fn auth_url(&self, path: &str) -> String {
        join(&self.auth_server_url, path)
    }

    pub fn spec_provider_url(&self, path: &str) -> String {
        join(&self.spec_provider_url, path)
    }
}

/// Joins a base URL and a path with exactly one `/` between them. The path is otherwise left
/// unchanged, so a trailing slash is preserved.
fn join(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> Config {
        Config::from_lookup(|key| {
            Some(match key {
                "IDENTITY_SERVER_URL" => "http://identity:8080".to_string(),
                "AUTH_SERVER_URL" => "http://auth:8081/".to_string(),
                "SPEC_PROVIDER_URL" => "http://spec:50051".to_string(),
                _ => return None,
            })
        })
    }

    #[test]
    fn reads_each_service_url() {
        let cfg = config();

        assert_eq!(
            cfg.identity_url("/v1/health"),
            "http://identity:8080/v1/health"
        );
        assert_eq!(cfg.auth_url("/health"), "http://auth:8081/health");
        assert_eq!(cfg.spec_provider_url("/"), "http://spec:50051/");
    }

    #[test]
    #[should_panic(expected = "AUTH_SERVER_URL must be set")]
    fn panics_naming_the_missing_variable() {
        Config::from_lookup(|key| (key != "AUTH_SERVER_URL").then(|| "http://x".to_string()));
    }

    #[test]
    fn tolerates_trailing_slash_on_base() {
        assert_eq!(config().auth_url("/par"), "http://auth:8081/par");
    }

    #[test]
    fn tolerates_missing_leading_slash_on_path() {
        assert_eq!(
            config().identity_url("v1/users"),
            "http://identity:8080/v1/users"
        );
    }

    #[test]
    fn preserves_trailing_slash_on_path() {
        assert_eq!(
            config().identity_url("/v1/users/"),
            "http://identity:8080/v1/users/"
        );
    }
}
