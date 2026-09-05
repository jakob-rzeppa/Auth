use sqlx::{prelude::FromRow, query_as};
use uuid::Uuid;

use crate::{domain::entity::user::User, persistence::get_connection};

pub enum FindByEmailUserError {
    InvalidData,
    DatabaseError,
}

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    has_temporary_password: bool,
}

pub async fn find_user_by_email(email: &str) -> Result<Option<User>, FindByEmailUserError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| FindByEmailUserError::DatabaseError)?;

    let row: Option<UserRow> = query_as!(
        UserRow,
        "SELECT id, email, password_hash, has_temporary_password FROM users WHERE email = $1",
        email
    )
    .fetch_optional(&mut *conn)
    .await
    .map_err(|error| {
        eprintln!("Unknown Database error: {:?}", error);
        FindByEmailUserError::DatabaseError
    })?;

    if let Some(row) = row {
        Ok(Some(
            User::new(
                row.id,
                row.email,
                row.password_hash,
                row.has_temporary_password,
                vec![],
            )
            .map_err(|error| {
                eprintln!("Database row violated user invariants: {:?}", error);
                FindByEmailUserError::InvalidData
            })?,
        ))
    } else {
        Ok(None)
    }
}
