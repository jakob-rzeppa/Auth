use sqlx::query;

use crate::{domain::entity::user::User, persistence::get_connection};

pub enum RegisterUserError {
    EmailAlreadyExists,
    DatabaseError,
}

const UNIQUE_VIOLATION: &str = "23505";

pub async fn register_user(user: &User) -> Result<(), RegisterUserError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| RegisterUserError::DatabaseError)?;

    query!(
        "INSERT INTO users (id, email, password_hash, has_temporary_password) VALUES ($1, $2, $3, $4)",
        user.id(), user.email(), user.password_hash(), user.has_temporary_password())
        .execute(&mut *conn)
        .await
        .map_err(|error| {
            if error
                .as_database_error()
                .and_then(|db_error| db_error.code())
                .as_deref()
                == Some(UNIQUE_VIOLATION)
            {
                RegisterUserError::EmailAlreadyExists
            } else {
                eprintln!("Unknown Database error: {:?}", error);
                RegisterUserError::DatabaseError
            }
        })?;

    Ok(())
}
