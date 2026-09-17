use redis::AsyncCommands;

use crate::{
    domain::entity::authorization_request::AuthorizationRequest,
    persistence::redis::get_redis_connection,
};

pub enum TakeParError {
    InvalidData,
    DatabaseError,
}

/// Atomically read and delete a pushed authorization request by its `request_uri`, enforcing one-time use.
/// Returns `None` if the PAR does not exist or has expired.
pub async fn take_par_by_request_uri(
    request_uri: &str,
) -> Result<Option<AuthorizationRequest>, TakeParError> {
    let mut conn = get_redis_connection()
        .await
        .map_err(|_| TakeParError::DatabaseError)?;

    let value: Option<String> =
        conn.get_del(format!("par:{request_uri}"))
            .await
            .map_err(|error| {
                eprintln!("Failed to take PAR: {:?}", error);
                TakeParError::DatabaseError
            })?;

    value
        .map(|value| {
            serde_json::from_str(&value).map_err(|error| {
                eprintln!("Invalid PAR data: {:?}", error);
                TakeParError::InvalidData
            })
        })
        .transpose()
}
