use uuid::Uuid;

use crate::{
    domain::user::{User, UserError},
    persistence::users::register::{RegisterUserError, register_user},
};

pub enum CreateUserApplicationError {
    InvalidEmail,
    EmailAlreadyExists,
    DatabaseError,
}

pub async fn create_user(email: String) -> Result<Uuid, CreateUserApplicationError> {
    let user = User::new(Uuid::new_v4(), email, vec![]).map_err(|e| match e {
        UserError::EmptyId => unreachable!("Uuid::new_v4() does not create a nil Uuid."),
        UserError::InvalidEmail => CreateUserApplicationError::InvalidEmail,
    })?;

    register_user(&user).await.map_err(|err| match err {
        RegisterUserError::EmailAlreadyExists => CreateUserApplicationError::EmailAlreadyExists,
        RegisterUserError::DatabaseError => CreateUserApplicationError::DatabaseError,
    })?;

    Ok(user.id())
}
