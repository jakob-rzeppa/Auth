use redis::AsyncCommands;

use crate::{
    domain::entity::authorization_request::AuthorizationRequest,
    persistence::redis::get_redis_connection,
};

pub enum PeekParError {
    InvalidData,
    DatabaseError,
}

/// Read a pushed authorization request by its `request_uri`.
/// Should only used for display purposes, as it does not enforce one-time use.
///
/// Returns `None` if the PAR does not exist or has expired.
pub async fn peek_par(request_uri: &str) -> Result<Option<AuthorizationRequest>, PeekParError> {
    let mut conn = get_redis_connection()
        .await
        .map_err(|_| PeekParError::DatabaseError)?;

    let value: Option<String> = conn
        .get(format!("par:{request_uri}"))
        .await
        .map_err(|error| {
            eprintln!("Failed to peek PAR: {:?}", error);
            PeekParError::DatabaseError
        })?;

    value
        .map(|value| {
            serde_json::from_str(&value).map_err(|error| {
                eprintln!("Invalid PAR data: {:?}", error);
                PeekParError::InvalidData
            })
        })
        .transpose()
}
