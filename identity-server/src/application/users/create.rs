use uuid::Uuid;

use crate::{
    application::password::{hash::hash_password, temporary::generate_temporary_password},
    domain::entity::user::{User, UserError},
    persistence::users::register::{RegisterUserError, register_user},
};

pub enum CreateUserApplicationError {
    InvalidUserName,
    UserNameAlreadyExists,
    PasswordHashingError,
    DatabaseError,
}

/// Creates a new user with the given user_name and display_name and returns the user's ID and a temporary password if successful.
pub async fn create_user(user_name: String) -> Result<(Uuid, String), CreateUserApplicationError> {
    let has_temporary_password = true;
    let temporary_password = generate_temporary_password();
    let password_hash = hash_password(&temporary_password)
        .map_err(|_| CreateUserApplicationError::PasswordHashingError)?;

    let display_name = build_initial_display_name(&user_name);

    let user = User::new(
        Uuid::new_v4(),
        user_name,
        display_name,
        password_hash,
        has_temporary_password,
        vec![],
    )
    .map_err(|e| match e {
        UserError::EmptyId => unreachable!("Uuid::new_v4() does not create a nil Uuid."),
        UserError::InvalidUserName => CreateUserApplicationError::InvalidUserName,
    })?;

    register_user(&user).await.map_err(|err| match err {
        RegisterUserError::UserNameAlreadyExists => {
            CreateUserApplicationError::UserNameAlreadyExists
        }
        RegisterUserError::DatabaseError => CreateUserApplicationError::DatabaseError,
    })?;

    Ok((user.id(), temporary_password))
}

fn build_initial_display_name(user_name: &str) -> String {
    let parts: Vec<&str> = user_name.split('.').collect();

    parts
        .into_iter()
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
