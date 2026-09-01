use crate::{domain::entity::user::User, persistence::get_connection};

pub enum RemoveUserError {
    DatabaseError,
}

pub async fn remove_user(user: &User) -> Result<(), RemoveUserError> {
    let mut conn = get_connection().await.map_err(|e| {
        eprintln!("Could not get database connection: {:?}", e);
        RemoveUserError::DatabaseError
    })?;

    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user.id())
        .execute(&mut *conn)
        .await
        .map_err(|error| {
            eprintln!("Unknown Database error: {:?}", error);
            RemoveUserError::DatabaseError
        })?;

    Ok(())
}
