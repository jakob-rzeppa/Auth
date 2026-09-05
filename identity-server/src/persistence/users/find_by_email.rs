use sqlx::query_as;

use crate::{
    domain::entity::user::User,
    persistence::{get_connection, users::UserRow},
};

pub enum FindByUserNameUserError {
    InvalidData,
    DatabaseError,
}

pub async fn find_user_by_user_name(
    user_name: &str,
) -> Result<Option<User>, FindByUserNameUserError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| FindByUserNameUserError::DatabaseError)?;

    let row: Option<UserRow> = query_as!(
        UserRow,
        "SELECT id, user_name, display_name, password_hash, has_temporary_password FROM users WHERE user_name = $1",
        user_name
    )
    .fetch_optional(&mut *conn)
    .await
    .map_err(|error| {
        eprintln!("Unknown Database error: {:?}", error);
        FindByUserNameUserError::DatabaseError
    })?;

    if let Some(row) = row {
        Ok(Some(
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
                FindByUserNameUserError::InvalidData
            })?,
        ))
    } else {
        Ok(None)
    }
}
