use uuid::Uuid;

use crate::{
    application::password::{
        hash::{HashPasswordError, hash_password},
        temporary::generate_temporary_password,
    },
    persistence::users::{
        find_by_id::{FindByIdUserError, find_user_by_id},
        save::{SaveUserError, save_user},
    },
};

pub enum ResetUserPasswordError {
    UserNotFound,
    HashingError,
    DatabaseError,
}

/// Resets the user's password to a new temporary password.
///
/// # Returns
///
/// The new temporary password in plain text.
pub async fn reset_user_password(user_id: Uuid) -> Result<String, ResetUserPasswordError> {
    let user = find_user_by_id(user_id).await.map_err(|e| match e {
        FindByIdUserError::DatabaseError => ResetUserPasswordError::DatabaseError,
        FindByIdUserError::InvalidData => ResetUserPasswordError::DatabaseError,
    })?;

    let Some(mut user) = user else {
        return Err(ResetUserPasswordError::UserNotFound);
    };

    // Hash the new temporary password
    let new_password = generate_temporary_password();
    let hashed_password = hash_password(&new_password).map_err(|e| match e {
        HashPasswordError::HashingError => ResetUserPasswordError::HashingError,
    })?;
    user.set_password_hash(hashed_password);

    save_user(&user).await.map_err(|e| match e {
        SaveUserError::DatabaseError => ResetUserPasswordError::DatabaseError,
        SaveUserError::UserNotFound => ResetUserPasswordError::DatabaseError,
        // This case should not happen since we just fetched the user, and don't change the email.
        SaveUserError::EmailAlreadyExists => ResetUserPasswordError::DatabaseError,
    })?;

    Ok(new_password)
}
