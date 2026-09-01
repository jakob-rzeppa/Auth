use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{domain::user::User, persistence::get_connection};

pub enum FindByIdUserError {
    InvalidData,
    DatabaseError,
}

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
}

pub async fn find_user_by_id(user_id: Uuid) -> Result<Option<User>, FindByIdUserError> {
    let mut conn = get_connection().await.map_err(|e| {
        eprintln!("Could not get database connection: {:?}", e);
        FindByIdUserError::DatabaseError
    })?;

    let row: Option<UserRow> = sqlx::query_as("SELECT id, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|error| {
            eprintln!("Unknown Database error: {:?}", error);
            FindByIdUserError::DatabaseError
        })?;

    row.map(|row| {
        User::new(row.id, row.email, vec![]).map_err(|error| {
            eprintln!("Database row violated user invariants: {:?}", error);
            FindByIdUserError::InvalidData
        })
    })
    .transpose()
}
