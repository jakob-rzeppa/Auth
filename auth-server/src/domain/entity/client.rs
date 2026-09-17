use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    id: Uuid,

    client_name: String,

    redirect_uris: Vec<String>,

    scopes: Vec<String>,
}

impl Client {
    pub fn new(
        id: Uuid,
        client_name: String,
        redirect_uris: Vec<String>,
        scopes: Vec<String>,
    ) -> Self {
        Self {
            id,
            client_name,
            redirect_uris,
            scopes,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn client_name(&self) -> &str {
        &self.client_name
    }

    pub fn has_redirect_uri(&self, redirect_uri: &str) -> bool {
        self.redirect_uris.contains(&redirect_uri.to_string())
    }

    pub fn has_scopes(&self, scopes: &[String]) -> bool {
        scopes.iter().all(|scope| self.scopes.contains(scope))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_redirect_uri() {
        let client = Client::new(
            Uuid::new_v4(),
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec!["read".to_string(), "write".to_string()],
        );

        assert!(client.has_redirect_uri("https://example.com/callback"));
        assert!(!client.has_redirect_uri("https://example.com/other"));
    }

    #[test]
    fn test_has_scopes() {
        let client = Client::new(
            Uuid::new_v4(),
            "Test Client".to_string(),
            vec!["https://example.com/callback".to_string()],
            vec!["read".to_string(), "write".to_string()],
        );

        assert!(client.has_scopes(&vec!["read".to_string()]));
        assert!(client.has_scopes(&vec!["read".to_string(), "write".to_string()]));
        assert!(!client.has_scopes(&vec!["delete".to_string()]));
    }
}
