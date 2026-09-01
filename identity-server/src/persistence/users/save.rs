use crate::{domain::entity::user::User, persistence::get_connection};

pub enum SaveUserError {
    UserNotFound,
    EmailAlreadyExists,
    DatabaseError,
}

const UNIQUE_VIOLATION: &str = "23505";

/// Save changes to a existing user in the database.
/// If the user does not exist, it will throw a error.
pub async fn save_user(user: &User) -> Result<(), SaveUserError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| SaveUserError::DatabaseError)?;

    let result = sqlx::query("UPDATE users SET email = $1 WHERE id = $2")
        .bind(user.email())
        .bind(user.id())
        .execute(&mut *conn)
        .await
        .map_err(|error| {
            if error
                .as_database_error()
                .and_then(|db_error| db_error.code())
                .as_deref()
                == Some(UNIQUE_VIOLATION)
            {
                SaveUserError::EmailAlreadyExists
            } else {
                eprintln!("Unknown Database error: {:?}", error);
                SaveUserError::DatabaseError
            }
        })?;

    if result.rows_affected() == 0 {
        eprintln!(
            "Tried to save changes to a unknown user with id {}.",
            user.id()
        );
        return Err(SaveUserError::UserNotFound);
    }

    Ok(())
}
