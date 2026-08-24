pub struct Client {
    pub id: String,
    pub secret: String,

    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}
