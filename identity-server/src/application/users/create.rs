use uuid::Uuid;

use crate::{
    application::password::{hash::hash_password, temporary::generate_temporary_password},
    domain::entity::user::{User, UserError},
    persistence::users::register::{RegisterUserError, register_user},
};

pub enum CreateUserApplicationError {
    InvalidEmail,
    EmailAlreadyExists,
    PasswordHashingError,
    DatabaseError,
}

/// Creates a new user with the given email and returns the user's ID and a temporary password if successful.
pub async fn create_user(email: String) -> Result<(Uuid, String), CreateUserApplicationError> {
    let has_temporary_password = true;
    let temporary_password = generate_temporary_password();
    let password_hash = hash_password(&temporary_password)
        .map_err(|_| CreateUserApplicationError::PasswordHashingError)?;

    let user = User::new(
        Uuid::new_v4(),
        email,
        password_hash,
        has_temporary_password,
        vec![],
    )
    .map_err(|e| match e {
        UserError::EmptyId => unreachable!("Uuid::new_v4() does not create a nil Uuid."),
        UserError::InvalidEmail => CreateUserApplicationError::InvalidEmail,
    })?;

    register_user(&user).await.map_err(|err| match err {
        RegisterUserError::EmailAlreadyExists => CreateUserApplicationError::EmailAlreadyExists,
        RegisterUserError::DatabaseError => CreateUserApplicationError::DatabaseError,
    })?;

    Ok((user.id(), temporary_password))
}
