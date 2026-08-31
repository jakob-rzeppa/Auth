use crate::{
    domain::privilege::Privilege,
    persistence::{DatabaseError, get_connection},
};

const UNIQUE_VIOLATION: &str = "23505";

pub async fn register_privilege(privilege: &Privilege) -> Result<(), DatabaseError> {
    let mut conn = get_connection().await?;

    sqlx::query("INSERT INTO privileges (id, name, description) VALUES ($1, $2, $3)")
        .bind(privilege.id())
        .bind(privilege.name())
        .bind(privilege.description())
        .execute(&mut *conn)
        .await
        .map_err(|error| {
            if error
                .as_database_error()
                .and_then(|db_error| db_error.code())
                .as_deref()
                == Some(UNIQUE_VIOLATION)
            {
                DatabaseError::DuplicateName
            } else {
                DatabaseError::QueryError(error)
            }
        })?;

    Ok(())
}
