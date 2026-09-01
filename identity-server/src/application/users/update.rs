use crate::persistence::users::{
    find_by_id::find_user_by_id,
    save::{SaveUserError, save_user},
};

pub enum UpdateUserError {
    UserNotFound,
    InvalidEmailFormat,
    EmailAlreadyExists,
    DatabaseError,
}

pub async fn update_user(
    user_id: uuid::Uuid,
    new_email: Option<String>,
) -> Result<(), UpdateUserError> {
    let user = find_user_by_id(user_id)
        .await
        .map_err(|_| UpdateUserError::DatabaseError)?;

    let Some(mut user) = user else {
        return Err(UpdateUserError::UserNotFound);
    };

    if let Some(email) = new_email {
        user.set_email(email)
            .map_err(|_| UpdateUserError::InvalidEmailFormat)?;
    }

    save_user(&user).await.map_err(|e| match e {
        SaveUserError::UserNotFound => UpdateUserError::DatabaseError,
        SaveUserError::EmailAlreadyExists => UpdateUserError::EmailAlreadyExists,
        SaveUserError::DatabaseError => UpdateUserError::DatabaseError,
    })?;

    Ok(())
}
