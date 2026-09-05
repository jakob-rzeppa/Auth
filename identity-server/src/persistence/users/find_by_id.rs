use sqlx::query_as;
use uuid::Uuid;

use crate::{
    domain::entity::user::User,
    persistence::{get_connection, users::UserRow},
};

pub enum FindByIdUserError {
    InvalidData,
    DatabaseError,
}

pub async fn find_user_by_id(user_id: Uuid) -> Result<Option<User>, FindByIdUserError> {
    let mut conn = get_connection()
        .await
        .map_err(|_| FindByIdUserError::DatabaseError)?;

    let row: Option<UserRow> = query_as!(
        UserRow,
        "SELECT id, user_name, display_name, password_hash, has_temporary_password FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&mut *conn)
    .await
    .map_err(|error| {
        eprintln!("Unknown Database error: {:?}", error);
        FindByIdUserError::DatabaseError
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
                FindByIdUserError::InvalidData
            })?,
        ))
    } else {
        Ok(None)
    }
}
