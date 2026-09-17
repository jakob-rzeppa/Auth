use redis::AsyncCommands;

use crate::{
    domain::entity::authorization_request::AuthorizationRequest,
    persistence::redis::get_redis_connection,
};

pub enum SaveParError {
    SerializationError,
    DatabaseError,
}

/// Save a pushed authorization request, to be automatically deleted by redis after `ttl_seconds`.
pub async fn save_par(
    request_uri: &str,
    par: AuthorizationRequest,
    ttl_seconds: u64,
) -> Result<(), SaveParError> {
    let mut conn = get_redis_connection()
        .await
        .map_err(|_| SaveParError::DatabaseError)?;

    let value = serde_json::to_string(&par).map_err(|error| {
        eprintln!("Failed to serialize PAR: {:?}", error);
        SaveParError::SerializationError
    })?;

    conn.set_ex::<_, _, ()>(key(request_uri), value, ttl_seconds)
        .await
        .map_err(|error| {
            eprintln!("Failed to save PAR: {:?}", error);
            SaveParError::DatabaseError
        })
}

fn key(request_uri: &str) -> String {
    format!("par:{request_uri}")
}
