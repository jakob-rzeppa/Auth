use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::entity::access_token::AccessToken;

pub mod find_by_token_hash;
pub mod register;
pub mod remove;

#[derive(FromRow)]
struct AccessTokenRow {
    token_hash: String,
    client_id: Uuid,
    iat: DateTime<Utc>,
    exp: DateTime<Utc>,
    scope: String,
}

impl AccessTokenRow {
    fn into_access_token(self) -> AccessToken {
        AccessToken::new(
            self.token_hash,
            self.client_id,
            self.iat,
            self.exp,
            self.scope,
        )
    }
}
