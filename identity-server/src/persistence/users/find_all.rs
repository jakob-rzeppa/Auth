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
        "SELECT id, user_name, display_name, password_hash, has_temporary_password FROM users"
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(|error| {
        eprintln!("Unknown Database error: {:?}", error);
        FindAllUsersError::DatabaseError
    })?;

    row.into_iter()
        .map(|row| {
            User::new(
                row.id,
                row.user_name,
                row.display_name,
                row.password_hash,
                row.has_temporary_password,
                vec![],
            )
            .map_err(|error| {
                eprintln!("Database row violated user invariants: {:?}", error);
                FindAllUsersError::InvalidData
            })
        })
        .collect::<Result<Vec<User>, FindAllUsersError>>()
}
