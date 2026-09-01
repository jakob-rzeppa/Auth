use crate::domain::entity::user::User;

pub enum UserHandlerError {
    InvalidPassword,
}

pub fn check_password_authentication(user: &User, password: &str) -> Result<(), UserHandlerError> {
    if user.password != password {
        return Err(UserHandlerError::InvalidPassword);
    }

    return Ok(());
}
