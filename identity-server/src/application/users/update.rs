use crate::persistence::users::{
    find_by_id::find_user_by_id,
    save::{SaveUserError, save_user},
};

pub enum UpdateUserError {
    UserNotFound,
    InvalidUserName,
    UserNameAlreadyExists,
    DatabaseError,
}

pub async fn update_user(
    user_id: uuid::Uuid,
    new_user_name: Option<String>,
    new_display_name: Option<String>,
) -> Result<(), UpdateUserError> {
    let user = find_user_by_id(user_id)
        .await
        .map_err(|_| UpdateUserError::DatabaseError)?;

    let Some(mut user) = user else {
        return Err(UpdateUserError::UserNotFound);
    };

    if let Some(email) = new_user_name {
        user.set_user_name(email)
            .map_err(|_| UpdateUserError::InvalidUserName)?;
    }

    if let Some(display_name) = new_display_name {
        user.set_display_name(display_name);
    }

    save_user(&user).await.map_err(|e| match e {
        SaveUserError::UserNotFound => UpdateUserError::DatabaseError,
        SaveUserError::UserNameAlreadyExists => UpdateUserError::UserNameAlreadyExists,
        SaveUserError::DatabaseError => UpdateUserError::DatabaseError,
    })?;

    Ok(())
}
