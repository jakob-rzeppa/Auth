use crate::{
    application::password::verify::{VerifyPasswordError, verify_password},
    domain::projection::user::FullUserProjection,
    persistence::users::find_by_email::{FindByUserNameUserError, find_user_by_user_name},
};

pub enum AuthenticateUserError {
    InvalidCredentials,
    UserNotFound,
    InternalError,
}

/// Authenticate a user.
///
/// # Returns
///
/// The `FullUserProjection` of the authenticated user and a boolean indicating whether the user must change their password.
pub async fn authenticate_user(
    user_name: &str,
    password: &str,
) -> Result<FullUserProjection, AuthenticateUserError> {
    let user = find_user_by_user_name(user_name)
        .await
        .map_err(|e| match e {
            FindByUserNameUserError::InvalidData => AuthenticateUserError::InternalError,
            FindByUserNameUserError::DatabaseError => AuthenticateUserError::InternalError,
        })?;

    let Some(user) = user else {
        return Err(AuthenticateUserError::UserNotFound);
    };

    verify_password(password, user.password_hash()).map_err(|e| match e {
        VerifyPasswordError::HashingError => AuthenticateUserError::InternalError,
        VerifyPasswordError::InvalidPassword => AuthenticateUserError::InvalidCredentials,
    })?;

    Ok(FullUserProjection::from(&user))
}
