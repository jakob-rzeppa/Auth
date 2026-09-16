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
}
