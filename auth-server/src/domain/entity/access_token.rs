use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct AccessToken {
    token_hash: String,

    client_id: Uuid,

    /// The time at which the token was issued, in UTC.
    iat: DateTime<Utc>,
    /// The time at which the token will expire, in UTC.
    exp: DateTime<Utc>,

    scope: String,
}

impl AccessToken {
    pub fn new(
        token_hash: String,
        client_id: Uuid,
        iat: DateTime<Utc>,
        exp: DateTime<Utc>,
        scope: String,
    ) -> Self {
        Self {
            token_hash,
            client_id,
            iat,
            exp,
            scope,
        }
    }
}

impl AccessToken {
    pub fn token_hash(&self) -> &str {
        &self.token_hash
    }

    pub fn client_id(&self) -> &Uuid {
        &self.client_id
    }

    pub fn iat(&self) -> &DateTime<Utc> {
        &self.iat
    }

    pub fn exp(&self) -> &DateTime<Utc> {
        &self.exp
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }
}
