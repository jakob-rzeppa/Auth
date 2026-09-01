use uuid::Uuid;

use crate::persistence::users::{find_by_id::find_user_by_id, remove::remove_user};

pub enum DeleteUserError {
    UserNotFound,
    DatabaseError,
}

pub async fn delete_user(user_id: Uuid) -> Result<(), DeleteUserError> {
    let user = find_user_by_id(user_id)
        .await
        .map_err(|_| DeleteUserError::DatabaseError)?;

    let Some(user) = user else {
        return Err(DeleteUserError::UserNotFound);
    };

    remove_user(&user)
        .await
        .map_err(|_| DeleteUserError::DatabaseError)?;

    Ok(())
}
