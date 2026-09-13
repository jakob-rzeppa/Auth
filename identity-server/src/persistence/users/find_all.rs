use sqlx::query_as;

use crate::{
    domain::entity::user::User,
    persistence::{get_connection, users::UserRow},
};

pub enum FindAllUsersError {
    InvalidData,
    DatabaseError,
}

/// Finds all users in the database and returns them as a vector of User entities.
pub async fn find_all_users() -> Result<Vec<User>, FindAllUsersError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| FindAllUsersError::DatabaseError)?;

    let row: Vec<UserRow> = query_as!(
        UserRow,
        "SELECT id, user_name, display_name, password_hash, has_temporary_password, roles FROM users"
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(|error| {
        eprintln!("Unknown Database error: {:?}", error);
        FindAllUsersError::DatabaseError
    })?;

    row.into_iter()
        .map(|row| {
            row.into_user().map_err(|error| {
                eprintln!("Invalid user row: {:?}", error);
                FindAllUsersError::InvalidData
            })
        })
        .collect::<Result<Vec<User>, FindAllUsersError>>()
}
