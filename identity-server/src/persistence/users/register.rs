use crate::{domain::entity::user::User, persistence::get_connection};

pub enum RegisterUserError {
    EmailAlreadyExists,
    DatabaseError,
}

const UNIQUE_VIOLATION: &str = "23505";

pub async fn register_user(user: &User) -> Result<(), RegisterUserError> {
    let mut conn = get_connection().await.map_err(|e| {
        eprintln!("Could not get database connection: {:?}", e);
        RegisterUserError::DatabaseError
    })?;

    sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2)")
        .bind(user.id())
        .bind(user.email())
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
